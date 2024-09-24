import { z } from "zod";
import { globalOptsSchema } from "../../common.ts";
import { loadProjectConfigPath, PROJECT_MANIFEST_PATH } from "../../../toolchain/project/mod.ts";
import { dirname, resolve } from "@std/path";
import { runTask } from "../../task.ts";
import { backendDataDir } from "../../../toolchain/project/project.ts";

export const inputSchema = z.object({}).merge(globalOptsSchema);

runTask({
	inputSchema,
	async run(input) {
		// Don't load project since that requires acquiring a lock on the project

		const projectRoot = dirname(loadProjectConfigPath(input));
		const dataDir = backendDataDir();
		const manifestPath = resolve(dataDir, PROJECT_MANIFEST_PATH);
		console.log(manifestPath);
	},
});
