import { z } from "zod";
import { globalOptsSchema } from "../../common.ts";
import { readConfig } from "../../../toolchain/config/project.ts";
import { loadProjectConfigPath } from "../../../toolchain/project/mod.ts";

export const optsSchema = z.object({}).merge(globalOptsSchema);

type Opts = z.infer<typeof optsSchema>;

export async function execute(opts: Opts) {
  // Don't load project since that requires acquiring a lock on the project

	const config = await readConfig(loadProjectConfigPath(opts));
	console.log(JSON.stringify(config, null, "\t"));
}
