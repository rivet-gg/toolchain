import { resolve } from "@std/path";
import { Project } from "../project/mod.ts";
import { DENO_JSON_PATH } from "../project/project.ts";
import { projectCachePath } from "../project/project.ts";

export async function generateDenoConfig(project: Project) {
	// Write config to user's project
	const userConfig = {
		"lint": {
			"include": ["src/"],
			"exclude": ["tests/"],
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
	await Deno.writeTextFile(resolve(project.path, "deno.json"), JSON.stringify(userConfig, null, 4));

	// Write config to project generated project
	const outputConfig = {
		"lint": {
			"include": ["src/"],
			"exclude": ["tests/"],
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
		"vendor": true,
	};
	await Deno.writeTextFile(projectCachePath(project, DENO_JSON_PATH), JSON.stringify(outputConfig, null, 4));
}
