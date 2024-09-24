import { z } from "zod";
import { globalOptsSchema } from "../../common.ts";
import { readConfig } from "../../../toolchain/config/project.ts";
import { loadProjectConfigPath } from "../../../toolchain/project/mod.ts";
import { runTask } from "../../task.ts";

export const inputSchema = z.object({}).merge(globalOptsSchema);

runTask({
  inputSchema,
  async run(input) {
	// Don't load project since that requires acquiring a lock on the project

	const config = await readConfig(loadProjectConfigPath(input));
	console.log(JSON.stringify(config, null, "\t"));

  }
})
