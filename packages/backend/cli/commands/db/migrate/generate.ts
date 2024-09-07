import { z } from "zod";
import { globalOptsSchema, initProject } from "../../../common.ts";
import { migrateGenerate } from "../../../../toolchain/migrate/generate.ts";
import { resolveModules } from "../../../util.ts";
import { UserError } from "../../../../toolchain/error/mod.ts";

export const optsSchema = z.object({
	modules: z.array(z.string()).default([]),
}).merge(globalOptsSchema);

type Opts = z.infer<typeof optsSchema>;

export async function execute(opts: Opts) {
	const project = await initProject(opts);
	const modules = resolveModules(project, opts.modules);

	for (const module of modules) {
		if (module.registry.isExternal) {
			throw new UserError(`Cannot run this command against external module: ${module.name}`);
		}
	}

	await migrateGenerate(project, modules);
}
