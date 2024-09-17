import { z } from "zod";
import { globalOptsSchema, initProject } from "../../common.ts";
import { metaPath } from "../../../toolchain/project/mod.ts";

export const optsSchema = z.object({}).merge(globalOptsSchema);

type Opts = z.infer<typeof optsSchema>;

export async function execute(opts: Opts) {
	const project = await initProject(opts);
  console.log(metaPath(project));
}
