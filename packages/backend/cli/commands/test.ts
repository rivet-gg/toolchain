import { z } from "zod";
import { resolve } from "@std/path";
import * as glob from "glob";
import { globalOptsSchema } from "../common.ts";
import { build, DbDriver, Format, MigrateMode, Runtime } from "../../toolchain/build/mod.ts";
import { watch } from "../../toolchain/watch/mod.ts";
import { Project } from "../../toolchain/project/mod.ts";
import { UserError } from "../../toolchain/error/mod.ts";
import { info } from "../../toolchain/term/status.ts";
import { migrateModeSchema } from "./../util.ts";
import { ensurePostgresRunning, getDefaultDatabaseUrl } from "../../toolchain/postgres/mod.ts";
import { denoExecutablePath } from "../../toolchain/utils/deno.ts";

export const optsSchema = z.object({
	build: z.boolean().default(true),
	check: z.boolean().default(true),
	strictSchemas: z.boolean().default(false),
	sdk: z.boolean().default(true),
	migrate: z.boolean().default(true),
	migrateMode: migrateModeSchema.default(MigrateMode.Dev),
	watch: z.boolean().default(false),
	filter: z.string().nullable(),
	modulesFilter: z.array(z.string()),
}).merge(globalOptsSchema);

type Opts = z.infer<typeof optsSchema>;

export async function execute(opts: Opts) {
	await watch({
		loadProjectOpts: opts,
		disableWatch: !opts.watch,
		fn: async (project: Project, signal: AbortSignal) => {
			await ensurePostgresRunning(project);

			// Build project
			if (opts.build) {
				await build(project, {
					runtime: Runtime.Deno,
					format: Format.Native,
					dbDriver: DbDriver.NodePostgres,
					strictSchemas: opts.strictSchemas,
					// This gets ran on `deno test`
					skipDenoCheck: true,
          sdk: opts.sdk ? {} : undefined,
					migrate: opts.migrate
						? {
							mode: opts.migrateMode,
						}
						: undefined,
					signal,
				});
			}

			// Determine args
			const args = [
				"--allow-env",
				"--allow-net",
				"--allow-read",
			];
			if (opts.check) args.push("--check");
			if (opts.filter) args.push(`--filter=${opts.filter}`);

			// Find test scripts
			const testingModules = [];
			let totalTestFiles = 0;
			for (const module of project.modules.values()) {
				// Filter modules
				if (opts.modulesFilter.length == 0) {
					// Only test local modules
					if (module.registry.isExternal) continue;
				} else {
					// Only test specified modules. This allows for testing remote modules.
					if (!opts.modulesFilter.includes(module.name)) continue;
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
          suggest: "See 'rivet create test --help' to create a test."
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
					"DATABASE_URL": await getDefaultDatabaseUrl(project),
					// Force color for test logs
					"RIVET_BACKEND_TERM_COLOR": Deno.env.get("RIVET_BACKEND_TERM_COLOR") ?? "always",
				},
			})
				.output();
			if (!cmd.success) throw new UserError("Tests failed.");
		},
	});
}
