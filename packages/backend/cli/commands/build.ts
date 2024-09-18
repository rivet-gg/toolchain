import { z } from "zod";
import { globalOptsSchema } from "../common.ts";
import { build, DbDriver, Format, MigrateMode, Runtime } from "../../toolchain/build/mod.ts";
import { watch } from "../../toolchain/watch/mod.ts";
import { Project } from "../../toolchain/project/mod.ts";
import { migrateModeSchema } from "../util.ts";

export const optsSchema = z.object({
	watch: z.boolean().default(false),
	runtime: z.enum([Runtime.Deno, Runtime.CloudflareWorkersPlatforms]).default(Runtime.Deno),
	outputFormat: z.enum([Format.Native, Format.Bundled]),
	dbDriver: z.enum([DbDriver.NodePostgres, DbDriver.NeonServerless, DbDriver.CloudflareHyperdrive]),
	sdk: z.boolean().default(true),
	migrate: z.boolean().default(true),
	migrateMode: migrateModeSchema.default(MigrateMode.Generate),
	strictSchemas: z.boolean().default(true),
}).merge(globalOptsSchema);

type Opts = z.infer<typeof optsSchema>;

export async function execute(opts: Opts) {
	// Defaults based on runtime
	if (opts.runtime == Runtime.Deno) {
		if (opts.outputFormat == undefined) opts.outputFormat = Format.Native;
		if (opts.dbDriver == undefined) opts.dbDriver = DbDriver.NodePostgres;
	} else if (opts.runtime == Runtime.CloudflareWorkersPlatforms) {
		if (opts.outputFormat == undefined) opts.outputFormat = Format.Bundled;
		if (opts.dbDriver == undefined) opts.dbDriver = DbDriver.NeonServerless;
	}

	// Validate
	if (opts.runtime == Runtime.CloudflareWorkersPlatforms) {
		if (opts.outputFormat != Format.Bundled) {
			throw new Error(
				`\`format\` must be "${Format.Bundled}" if \`runtime\` is "${Runtime.CloudflareWorkersPlatforms}".`,
			);
		}
		if (opts.dbDriver != DbDriver.NeonServerless && opts.dbDriver != DbDriver.CloudflareHyperdrive) {
			throw new Error(
				`\`db-driver\` must be "${DbDriver.NeonServerless}" or "${DbDriver.CloudflareHyperdrive}" if \`runtime\` is "${Runtime.CloudflareWorkersPlatforms}".`,
			);
		}
	}
	if (opts.runtime == Runtime.Deno) {
		if (opts.outputFormat != Format.Native) {
			throw new Error(
				`\`format\` must be "${Format.Native}" if \`runtime\` is "${Runtime.Deno}".`,
			);
		}
	}

	await watch({
		loadProjectOpts: opts,
		disableWatch: !opts.watch,
		async fn(project: Project, signal: AbortSignal) {
			await build(project, {
				format: opts.outputFormat!,
				runtime: opts.runtime,
				dbDriver: opts.dbDriver!,
				strictSchemas: opts.strictSchemas,
				skipDenoCheck: false,
        sdk: opts.sdk ? {} : undefined,
				migrate: opts.migrate
					? {
						mode: opts.migrateMode,
					}
					: undefined,
				signal,
			});
		},
	});
}
