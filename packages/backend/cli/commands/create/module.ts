import { z } from "zod";
import { templateModule } from "../../../toolchain/template/module.ts";
import { Casing } from "../../../toolchain/types/identifiers/defs.ts";
import { validateIdentifier } from "../../../toolchain/types/identifiers/mod.ts";
import { globalOptsSchema, initProject } from "../../common.ts";

export const optsSchema = z.object({
	module: z.string(),
}).merge(globalOptsSchema);

type Opts = z.infer<typeof optsSchema>;

export async function execute(opts: Opts) {
	validateIdentifier(opts.module, Casing.Snake);
	const project = await initProject(opts);
	await templateModule(project, opts.module);
}
