import { z } from "zod";
import { globalOptsSchema, initProject } from "../common.ts";
import { cleanProject } from "../../toolchain/project/project.ts";

export const optsSchema = globalOptsSchema;

type Opts = z.infer<typeof optsSchema>;

export async function execute(opts: Opts) {
	const project = await initProject(opts);
	await cleanProject(project);
}
