import { resolve } from "@std/path";
import { Project } from "../project/mod.ts";
import { DENO_JSON_PATH } from "../project/project.ts";
import { projectCachePath } from "../project/project.ts";
import { autoGenHeader } from "./misc.ts";
import { exists } from "@std/fs";

export async function generateDenoConfig(project: Project) {
	// Config that will be used as a base for the user config and the generated config
	const denoConfig = {
		"lint": {
			"rules": {
				"exclude": ["no-empty-interface", "no-explicit-any", "require-await"],
			},
		},
		"fmt": {
			"useTabs": true,
		},
		"imports": {
			// HACK: Cloudflare requires this to be specified specifically as
			// `cloudflare:workers` so we can't use `npm:` syntax. We bind this
			// to the Cloudflare types instead.
			"cloudflare:workers": "npm:@cloudflare/workers-types",
		},
	};
	const denoConfigJson = JSON.stringify(denoConfig, null, "\t");
	const denoConfigWithHeader = `${autoGenHeader()}\n\n${denoConfigJson}`;

	// Write config to user's local registries. We do this in order to enable IDE
	// support.
	for (const registry of project.registries.values()) {
		if (registry.isExternal) continue;
    if (!await exists(registry.path, { isDirectory: true })) continue;

    await Deno.writeTextFile(resolve(registry.path, "deno.jsonc"), denoConfigWithHeader);
	}

	// Write config to project generated project
	await Deno.writeTextFile(projectCachePath(project, DENO_JSON_PATH), denoConfigWithHeader);
}
