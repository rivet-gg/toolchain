import { z } from "zod";
import { resolve } from "@std/path";
import * as glob from "glob";
import { globalOptsSchema } from "../common.ts";
import { build, DbDriver, Format, MigrateMode, Runtime } from "../../toolchain/build/mod.ts";
import { watch } from "../../toolchain/watch/mod.ts";
import { DENO_JSON_PATH, DENO_LOCK_PATH, Project, projectDataPath } from "../../toolchain/project/mod.ts";
import { UserError } from "../../toolchain/error/mod.ts";
import { info } from "../../toolchain/term/status.ts";
import { migrateModeSchema } from "./../util.ts";
import { denoExecutablePath } from "../../toolchain/utils/deno.ts";
import { getDatabaseUrl } from "../../toolchain/postgres.ts";
import { runTask } from "../task.ts";

export const inputSchema = z.object({
	build: z.boolean().default(true),
	check: z.boolean().default(true),
	strictSchemas: z.boolean().default(true),
	sdk: z.boolean().default(true),
	migrate: z.boolean().default(true),
	migrateMode: migrateModeSchema.default(MigrateMode.Dev),
	watch: z.boolean().default(false),
	filter: z.string().nullable(),
	modulesFilter: z.array(z.string()),
}).merge(globalOptsSchema);

runTask({
	inputSchema,
	async run(input) {
		await watch({
			loadProjectOpts: input,
			disableWatch: !input.watch,
			fn: async (project: Project, signal: AbortSignal) => {
				// Build project
				if (input.build) {
					await build(project, {
						runtime: Runtime.Deno,
						format: Format.Native,
						dbDriver: DbDriver.NodePostgres,
						strictSchemas: input.strictSchemas,
						// This gets ran on `deno test`
						skipDenoCheck: true,
						sdk: input.sdk ? {} : undefined,
						migrate: input.migrate
							? {
								mode: input.migrateMode,
							}
							: undefined,
						signal,
					});
				}

				// Determine args
				const args = [
					"--config",
					projectDataPath(project, DENO_JSON_PATH),
					"--lock",
					projectDataPath(project, DENO_LOCK_PATH),
					"--allow-env",
					"--allow-net",
					"--allow-read",
				];
				if (input.check) args.push("--check");
				if (input.filter) args.push(`--filter=${input.filter}`);

				// Find test scripts
				const testingModules = [];
				let totalTestFiles = 0;
				for (const module of project.modules.values()) {
					// Filter modules
					if (input.modulesFilter.length == 0) {
						// Only test local modules
						if (module.registry.isExternal) continue;
					} else {
						// Only test specified modules. This allows for testing remote modules.
						if (!input.modulesFilter.includes(module.name)) continue;
					}

					testingModules.push(module.name);

					// Test all modules or filter module tests
					const testPaths = (await glob.glob("*.ts", {
						cwd: resolve(module.path, "tests"),
					}))
						.map((path) => resolve(module.path, "tests", path));
					totalTestFiles += testPaths.length;
					args.push(...testPaths);
				}

				if (testingModules.length == 0) {
					info("Finished", "No modules to test");
					return;
				}

				if (totalTestFiles == 0) {
					throw new UserError("No test files", {
						suggest: "See 'rivet create test --help' to create a test.",
					});
				}

				// Run tests
				info("Testing", testingModules.join(", "));
				const cmd = await new Deno.Command(denoExecutablePath(), {
					args: [
						"test",
						...args,
					],
					stdout: "inherit",
					stderr: "inherit",
					signal,
					env: {
						"DATABASE_URL": getDatabaseUrl(project),
						// Force color for test logs
						"RIVET_BACKEND_TERM_COLOR": Deno.env.get("RIVET_BACKEND_TERM_COLOR") ?? "always",
					},
				})
					.output();
				if (!cmd.success) throw new UserError("Tests failed.");
			},
		});
	},
});
