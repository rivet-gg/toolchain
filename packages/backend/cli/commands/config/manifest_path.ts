import { z } from "zod";
import { globalOptsSchema } from "../../common.ts";
import { computeProjectCachePath, loadProjectConfigPath, PROJECT_MANIFEST_PATH } from "../../../toolchain/project/mod.ts";
import { dirname, resolve } from "@std/path";

export const optsSchema = z.object({}).merge(globalOptsSchema);

type Opts = z.infer<typeof optsSchema>;

export async function execute(opts: Opts) {
  // Don't load project since that requires acquiring a lock on the project

	const projectRoot = dirname(loadProjectConfigPath(opts));
	const cachePath = await computeProjectCachePath(projectRoot);
  const manifestPath = resolve(cachePath, PROJECT_MANIFEST_PATH)
	console.log(manifestPath);
}
