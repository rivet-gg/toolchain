import { z } from "zod";
import { globalOptsSchema } from "../common.ts";
import { build, DbDriver, Format, MigrateMode, Runtime } from "../../toolchain/build/mod.ts";
import { watch } from "../../toolchain/watch/mod.ts";
import { Project } from "../../toolchain/project/mod.ts";
import { migrateModeSchema } from "../util.ts";
import { runTask } from "../task.ts";

export const inputSchema = z.object({
	watch: z.boolean().default(false),
	runtime: z.enum([Runtime.Deno, Runtime.CloudflareWorkersPlatforms]).default(Runtime.Deno),
	outputFormat: z.enum([Format.Native, Format.Bundled]),
	dbDriver: z.enum([DbDriver.NodePostgres, DbDriver.NeonServerless, DbDriver.CloudflareHyperdrive]),
	sdk: z.boolean().default(true),
	migrate: z.boolean().default(true),
	migrateMode: migrateModeSchema.default(MigrateMode.Generate),
	strictSchemas: z.boolean().default(true),
}).merge(globalOptsSchema);

runTask({
	inputSchema,
	async run(input) {
		// Defaults based on runtime
		if (input.runtime == Runtime.Deno) {
			if (input.outputFormat == undefined) input.outputFormat = Format.Native;
			if (input.dbDriver == undefined) input.dbDriver = DbDriver.NodePostgres;
		} else if (input.runtime == Runtime.CloudflareWorkersPlatforms) {
			if (input.outputFormat == undefined) input.outputFormat = Format.Bundled;
			if (input.dbDriver == undefined) input.dbDriver = DbDriver.NeonServerless;
		}

		// Validate
		if (input.runtime == Runtime.CloudflareWorkersPlatforms) {
			if (input.outputFormat != Format.Bundled) {
				throw new Error(
					`\`format\` must be "${Format.Bundled}" if \`runtime\` is "${Runtime.CloudflareWorkersPlatforms}".`,
				);
			}
			if (input.dbDriver != DbDriver.NeonServerless && input.dbDriver != DbDriver.CloudflareHyperdrive) {
				throw new Error(
					`\`db-driver\` must be "${DbDriver.NeonServerless}" or "${DbDriver.CloudflareHyperdrive}" if \`runtime\` is "${Runtime.CloudflareWorkersPlatforms}".`,
				);
			}
		}
		if (input.runtime == Runtime.Deno) {
			if (input.outputFormat != Format.Native) {
				throw new Error(
					`\`format\` must be "${Format.Native}" if \`runtime\` is "${Runtime.Deno}".`,
				);
			}
		}

		await watch({
			loadProjectOpts: input,
			disableWatch: !input.watch,
			async fn(project: Project, signal: AbortSignal) {
				await build(project, {
					format: input.outputFormat!,
					runtime: input.runtime,
					dbDriver: input.dbDriver!,
					strictSchemas: input.strictSchemas,
					skipDenoCheck: false,
					sdk: input.sdk ? {} : undefined,
					migrate: input.migrate
						? {
							mode: input.migrateMode,
						}
						: undefined,
					signal,
				});
			},
		});
	},
});
