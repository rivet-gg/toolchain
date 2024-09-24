import { globalOptsSchema, initProject } from "../../common.ts";
import { resolve } from "@std/path";
import { fetchAndResolveModule } from "../../../toolchain/project/mod.ts";
import { ProjectModuleConfig } from "../../../toolchain/config/project.ts";
import { UserError } from "../../../toolchain/error/mod.ts";
import { z } from "zod";
import { runTask } from "../../task.ts";

export const inputSchema = z.object({
	moduleName: z.string(),
	registry: z.string().nullable(),
}).merge(globalOptsSchema);

runTask({
  inputSchema,
  async run(input) {
	const project = await initProject(input);

	// Ensure not already installed
	if (input.moduleName in project.config.modules) {
		throw new UserError(`Module \`${input.moduleName}\` is already installed`);
	}

	// Attempt to fetch module
	const moduleConfig: ProjectModuleConfig = {};
	if (input.registry) moduleConfig.registry = input.registry;
	await fetchAndResolveModule(project.path, project.configPath, project.registries, input.moduleName, moduleConfig);

	// Add to rivet.json
	const newConfig = structuredClone(project.config);
	newConfig.modules[input.moduleName] = moduleConfig;
	await Deno.writeTextFile(
		resolve(project.path, "rivet.json"),
		JSON.stringify(newConfig, null, "\t"),
	);

  }
});

