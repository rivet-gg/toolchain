import { z } from "zod";
import { globalOptsSchema } from "../common.ts";
import { build, DbDriver, Format, MigrateMode, Runtime } from "../../toolchain/build/mod.ts";
import { watch } from "../../toolchain/watch/mod.ts";
import { Project } from "../../toolchain/project/mod.ts";
import { InternalError } from "../../toolchain/error/mod.ts";
import { ENTRYPOINT_PATH, projectCachePath } from "../../toolchain/project/project.ts";
import { migrateModeSchema } from "./../util.ts";
import { createAndStartProjectInternalApiRouter, InternalState, State } from "../../toolchain/internal_api/mod.ts";
import { denoExecutablePath } from "../../toolchain/utils/deno.ts";
import { getDatabaseUrl } from "../../toolchain/postgres.ts";
import { runTask } from "../task.ts";

export const inputSchema = z.object({
	build: z.boolean().default(true),
	check: z.boolean().default(true),
	strictSchemas: z.boolean().default(true),
	watch: z.boolean().default(true),
	sdk: z.boolean().default(true),
	migrate: z.boolean().default(true),
	migrateMode: migrateModeSchema.default(MigrateMode.Dev),
	nonInteractive: z.boolean().default(false),
}).merge(globalOptsSchema);

runTask({
  inputSchema,
async run(input) {
	// Start internal router once we receive an event from `watch`
	const internalState = new InternalState();
	let startedInternalRouter = false;
	const setInternalState = (state: State) => {
		// Start internal router if needed
		if (!startedInternalRouter) {
			createAndStartProjectInternalApiRouter(internalState);
			startedInternalRouter = true;
		}

		// Set state
		internalState.set(state);
	};

	await watch({
		loadProjectOpts: input,
		disableWatch: !input.watch,
		onError: (project, error) => {
			if (project) setInternalState({ value: "failure", project, error });
		},
		onFileChange: (project) => {
			if (project) setInternalState({ value: "building", project });
		},
		onProjectChange(project) {
			setInternalState({ value: "building", project });
		},
		async fn(project: Project, signal: AbortSignal) {
			// Build project
			if (input.build) {
				await build(project, {
					runtime: Runtime.Deno,
					format: Format.Native,
					dbDriver: DbDriver.NodePostgres,
					strictSchemas: input.strictSchemas,
					// This gets ran on `deno run`
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
			internalState.set({ value: "success", project });

			// Determine args
			const args = [
				"--allow-env",
				"--allow-net",
				"--allow-read",
			];
			if (input.check) args.push("--check");

			// Run entrypoint
			const entrypointPath = projectCachePath(project, ENTRYPOINT_PATH);
			const cmd = await new Deno.Command(denoExecutablePath(), {
				args: [
					"run",
					...args,
					entrypointPath,
				],
				stdout: "inherit",
				stderr: "inherit",
				signal,
				env: {
					"DATABASE_URL": getDatabaseUrl(project),
				},
			})
				.output();
			if (!signal.aborted && !cmd.success) throw new InternalError("Entrypoint failed", { path: entrypointPath });
		},
	});
}
});
