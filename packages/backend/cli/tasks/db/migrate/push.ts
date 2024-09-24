import { z } from "zod";
import { globalOptsSchema, initProject } from "../../../common.ts";
import { migratePush } from "../../../../toolchain/migrate/push.ts";
import { resolveModules } from "../../../util.ts";
import { runTask } from "../../../task.ts";

export const inputSchema = z.object({
	modules: z.array(z.string()).default([]),
}).merge(globalOptsSchema);

runTask({
  inputSchema,
  async run(input) {

	const project = await initProject(input);
	const modules = resolveModules(project, input.modules);

	await migratePush(project, modules);
  }
})

