import { z } from "zod";
import { getDefaultDatabaseUrl } from "../../../toolchain/postgres/mod.ts";
import { globalOptsSchema, initProject } from "../../common.ts";

export const optsSchema = z.object({}).merge(globalOptsSchema);

type Opts = z.infer<typeof optsSchema>;

export async function execute(opts: Opts) {
	const project = await initProject(opts);
	const dbUrl = await getDefaultDatabaseUrl(project);
	console.log(dbUrl);
}
