import { z } from "zod";
import { loadProject, Project } from "../toolchain/project/mod.ts";

export const globalOptsSchema = z.object({
	/** Path to the project root or project config. */
	project: z.string().nullable(),
}).catchall(z.unknown());

export type GlobalOpts = z.infer<typeof globalOptsSchema>;

export async function initProject(opts: GlobalOpts): Promise<Project> {
	const project = await loadProject({ project: opts.project });
	return project;
}
