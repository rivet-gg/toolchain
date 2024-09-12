import { z } from "zod";
import { globalOptsSchema, initProject } from "../common.ts";
import { build, DbDriver, Format, MigrateMode, Runtime } from "../../toolchain/build/mod.ts";
import { watch } from "../../toolchain/watch/mod.ts";
import { Project } from "../../toolchain/project/mod.ts";
import { InternalError } from "../../toolchain/error/mod.ts";
import { ENTRYPOINT_PATH, projectCachePath } from "../../toolchain/project/project.ts";
import { migrateModeSchema } from "./../util.ts";
import { ensurePostgresRunning, getDefaultDatabaseUrl } from "../../toolchain/postgres/mod.ts";
import { InternalState } from "../../toolchain/internal_api/state.ts";
import { createAndStartProjectInternalApiRouter } from "../../toolchain/internal_api/mod.ts";

export const optsSchema = z.object({
	build: z.boolean().default(true),
	check: z.boolean().default(true),
	strictSchemas: z.boolean().default(true),
	watch: z.boolean().default(true),
	sdk: z.boolean().default(true),
	migrate: z.boolean().default(true),
	migrateMode: migrateModeSchema.default(MigrateMode.Dev),
	nonInteractive: z.boolean().default(false),
}).merge(globalOptsSchema);

type Opts = z.infer<typeof optsSchema>;

export async function execute(opts: Opts) {
	const project = await initProject(opts);

	const internalState = new InternalState();
	internalState.set({ value: "building", project });

	createAndStartProjectInternalApiRouter(internalState);
	await watch({
		loadProjectOpts: opts,
		disableWatch: !opts.watch,
		onError: (error) => {
			internalState.set({ value: "failure", project, error });
		},
		onFileChange: () => {
			internalState.set({ value: "building", project });
		},
		onProjectChange(project) {
			internalState.set({ value: "building", project });
		},
		async fn(project: Project, signal: AbortSignal) {
			await ensurePostgresRunning(project);

			// Build project
			if (opts.build) {
				await build(project, {
					runtime: Runtime.Deno,
					format: Format.Native,
					dbDriver: DbDriver.NodePostgres,
					strictSchemas: opts.strictSchemas,
					// This gets ran on `deno run`
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
			internalState.set({ value: "success", project });

			// Determine args
			const args = [
				"--allow-env",
				"--allow-net",
				"--allow-read",
			];
			if (opts.check) args.push("--check");

			// Run entrypoint
			const entrypointPath = projectCachePath(project, ENTRYPOINT_PATH);
			const cmd = await new Deno.Command("deno", {
				args: [
					"run",
					...args,
					entrypointPath,
				],
				stdout: "inherit",
				stderr: "inherit",
				signal,
				env: {
					"DATABASE_URL": await getDefaultDatabaseUrl(project),
				},
			})
				.output();
			if (!signal.aborted && !cmd.success) throw new InternalError("Entrypoint failed", { path: entrypointPath });
		},
	});
}
