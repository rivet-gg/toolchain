import { z } from "zod";
import { globalOptsSchema } from "../../common.ts";
import { readConfig } from "../../../toolchain/config/project.ts";
import { loadProjectConfigPath } from "../../../toolchain/project/mod.ts";

export const optsSchema = z.object({}).merge(globalOptsSchema);

type Opts = z.infer<typeof optsSchema>;

export async function execute(opts: Opts) {
	const config = await readConfig(loadProjectConfigPath(opts));
	console.log(JSON.stringify(config, null, "\t"));
}
