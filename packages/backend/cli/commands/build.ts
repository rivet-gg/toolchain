import { z } from "zod";
import { globalOptsSchema } from "../common.ts";
import { build, DbDriver, Format, MigrateMode, Runtime } from "../../toolchain/build/mod.ts";
import { watch } from "../../toolchain/watch/mod.ts";
import { Project } from "../../toolchain/project/mod.ts";
import { migrateModeSchema } from "../util.ts";

export const optsSchema = z.object({
	watch: z.boolean().default(false).nullable(),
	runtime: z.enum([Runtime.Deno, Runtime.CloudflareWorkersPlatforms]).default(Runtime.Deno).nullable(),
	outputFormat: z.enum([Format.Native, Format.Bundled]).nullable(),
	dbDriver: z.enum([DbDriver.NodePostgres, DbDriver.NeonServerless, DbDriver.CloudflareHyperdrive]).nullable(),
	migrate: z.boolean().default(true).nullable(),
	migrateMode: migrateModeSchema.default(MigrateMode.Generate).nullable(),
	strictSchemas: z.boolean().default(true).nullable(),
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
				`\`format\` must be "bundled" if \`runtime\` is "cloudflare-workers-platforms".`,
			);
		}
		if (opts.dbDriver != DbDriver.NeonServerless && opts.dbDriver != DbDriver.CloudflareHyperdrive) {
			throw new Error(
				`\`db-driver\` must be "neon-serverless" or "cloudflare-hyperdrive" if \`runtime\` is "cloudflare-workers-platforms".`,
			);
		}
	}
	if (opts.runtime == Runtime.Deno) {
		if (opts.outputFormat != Format.Native) {
			throw new Error(
				`\`format\` must be "native" if \`runtime\` is "deno".`,
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
