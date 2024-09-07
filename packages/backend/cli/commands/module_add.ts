import { globalOptsSchema, initProject } from "../common.ts";
import { resolve } from "@std/path";
import { fetchAndResolveModule } from "../../toolchain/project/mod.ts";
import { ProjectModuleConfig } from "../../toolchain/config/project.ts";
import { UserError } from "../../toolchain/error/mod.ts";
import { z } from "zod";

export const optsSchema = z.object({
	moduleName: z.string(),
	registry: z.string().optional(),
}).merge(globalOptsSchema);

type Opts = z.infer<typeof optsSchema>;

export async function execute(opts: Opts) {
	const project = await initProject(opts);

	// Ensure not already installed
	if (opts.moduleName in project.config.modules) {
		throw new UserError(`Module \`${opts.moduleName}\` is already installed`);
	}

	// Attempt to fetch module
	const moduleConfig: ProjectModuleConfig = {};
	if (opts.registry) moduleConfig.registry = opts.registry;
	await fetchAndResolveModule(project.path, project.configPath, project.registries, opts.moduleName, moduleConfig);

	// Add to backend.json
	const newConfig = structuredClone(project.config);
	newConfig.modules[opts.moduleName] = moduleConfig;
	await Deno.writeTextFile(
		resolve(project.path, "backend.json"),
		JSON.stringify(newConfig, null, "\t"),
	);
}
