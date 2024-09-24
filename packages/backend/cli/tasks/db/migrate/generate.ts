import { z } from "zod";
import { globalOptsSchema, initProject } from "../../../common.ts";
import { migrateGenerate } from "../../../../toolchain/migrate/generate.ts";
import { resolveModules } from "../../../util.ts";
import { UserError } from "../../../../toolchain/error/mod.ts";
import { runTask } from "../../../task.ts";

export const inputSchema = z.object({
	modules: z.array(z.string()).default([]),
}).merge(globalOptsSchema);

runTask({
	inputSchema,
	async run(input) {
		const project = await initProject(input);
		const modules = resolveModules(project, input.modules);

		for (const module of modules) {
			if (module.registry.isExternal) {
				throw new UserError(`Cannot run this command against external module: ${module.name}`);
			}
		}

		await migrateGenerate(project, modules);
	},
});
