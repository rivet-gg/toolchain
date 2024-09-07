import { z } from "zod";
import { globalOptsSchema, initProject } from "../common.ts";
import { generateSdk } from "../../toolchain/sdk/generate.ts";
import { SdkTarget } from "../../toolchain/sdk/generate.ts";
import { build, DbDriver, Format, Runtime } from "../../toolchain/build/mod.ts";

export const optsSchema = z.object({
	target: z.enum([SdkTarget.TypeScript, SdkTarget.Unity, SdkTarget.Godot]),
	output: z.string().default("./sdk"),
}).merge(globalOptsSchema);

type Opts = z.infer<typeof optsSchema>;

export async function execute(opts: Opts) {
	const project = await initProject(opts);

	// Build with schemas
	await build(project, {
		runtime: Runtime.Deno,
		format: Format.Native,
		dbDriver: DbDriver.NodePostgres,
		strictSchemas: true,
		skipDenoCheck: true,
	});

	// Generate SDK
	await generateSdk(project, opts.target, opts.output);
}
