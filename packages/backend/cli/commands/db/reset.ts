import { z } from "zod";
import { dbReset } from "../../../toolchain/migrate/reset.ts";
import { globalOptsSchema, initProject } from "../../common.ts";
import { resolveModules } from "../../util.ts";

export const optsSchema = z.object({
	modules: z.array(z.string()).default([]),
}).merge(globalOptsSchema);

type Opts = z.infer<typeof optsSchema>;

export async function execute(opts: Opts) {
	const project = await initProject(opts);
	const modules = resolveModules(project, opts.modules);

	await dbReset(project, modules);
}
