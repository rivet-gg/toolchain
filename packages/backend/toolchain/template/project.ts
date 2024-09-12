import { resolve } from "@std/path";
import { ProjectConfig } from "../config/project.ts";

export async function templateProject(rootPath: string) {
	await Deno.mkdir(rootPath, { recursive: true });

	// Create rivet.json
	const defaultBackend: ProjectConfig = {
		registries: {
			local: {
				local: {
					directory: "modules",
				},
			},
		},
		modules: {
			rate_limit: {},
			tokens: {},
		},
	};
	await Deno.writeTextFile(
		resolve(rootPath, "rivet.json"),
		JSON.stringify(defaultBackend, null, "\t"),
	);
}
