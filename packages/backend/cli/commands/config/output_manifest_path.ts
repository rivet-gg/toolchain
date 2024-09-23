import { z } from "zod";
import { globalOptsSchema, initProject } from "../../common.ts";
import { OUTPUT_MANIFEST_PATH } from "../../../toolchain/project/mod.ts";
import { projectCachePath } from "../../../toolchain/project/project.ts";

export const optsSchema = z.object({}).merge(globalOptsSchema);

type Opts = z.infer<typeof optsSchema>;

export async function execute(opts: Opts) {
	const project = await initProject(opts);
	console.log(projectCachePath(project, OUTPUT_MANIFEST_PATH));
}
