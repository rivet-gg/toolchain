import { z } from "zod";
import { templateTest } from "../../toolchain/template/test.ts";
import { Casing } from "../../toolchain/types/identifiers/defs.ts";
import { validateIdentifier } from "../../toolchain/types/identifiers/mod.ts";
import { globalOptsSchema, initProject } from "../common.ts";

export const optsSchema = z.object({
	module: z.string(),
	test: z.string(),
}).merge(globalOptsSchema);

type Opts = z.infer<typeof optsSchema>;

export async function execute(opts: Opts) {
	validateIdentifier(opts.module, Casing.Snake);
	validateIdentifier(opts.test, Casing.Snake);

	await templateTest(await initProject(opts), opts.module, opts.test);
}
