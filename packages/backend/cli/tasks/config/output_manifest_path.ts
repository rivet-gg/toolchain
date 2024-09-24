import { z } from "zod";
import { globalOptsSchema, initProject } from "../../common.ts";
import { OUTPUT_MANIFEST_PATH } from "../../../toolchain/project/mod.ts";
import {
	computeProjectCachePath,
	loadProjectConfigPath,
} from "../../../toolchain/project/project.ts";
import { dirname, resolve } from "@std/path";
import { runTask } from "../../task.ts";

export const inputSchema = z.object({}).merge(globalOptsSchema);

runTask({
  inputSchema,
  async run(input) {
	// Don't load project since that requires acquiring a lock on the project

	const projectRoot = dirname(loadProjectConfigPath(input));
	const cachePath = await computeProjectCachePath(projectRoot);
	const manifestPath = resolve(cachePath, OUTPUT_MANIFEST_PATH);
	console.log(manifestPath);

  }
})
