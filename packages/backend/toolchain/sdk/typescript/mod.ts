import { camelify, pascalify } from "../../../case_conversion/mod.ts";
import { dirname, resolve } from "@std/path";
import * as glob from "glob";
import { GeneratedCodeBuilder, Lang } from "../../build/gen/code_builder.ts";
import { Project } from "../../project/mod.ts";
import { autoGenHeader } from "../../build/misc.ts";
import { BACKEND_ROOT } from "../../utils/paths.ts";

// List of reserved JavaScript words to avoid naming collisions
const RESERVED_WORDS = [
	"break",
	"case",
	"catch",
	"class",
	"const",
	"continue",
	"debugger",
	"default",
	"delete",
	"do",
	"else",
	"export",
	"extends",
	"finally",
	"for",
	"function",
	"if",
	"import",
	"in",
	"instanceof",
	"new",
	"return",
	"super",
	"switch",
	"this",
	"throw",
	"try",
	"typeof",
	"var",
	"void",
	"while",
	"with",
	"yield",
];

export async function generateTypescript(project: Project, sdkGenPath: string) {
	await copyBase(sdkGenPath);
	await generateBackendAndModules(project, sdkGenPath);
	await generateModuleAddons(project, sdkGenPath);
}

export async function copyBase(sdkGenPath: string) {
	const sourceDir = resolve(BACKEND_ROOT, "dynamic", "sdk", "typescript");
	const paths = await glob.glob("**/*.{ts,json}", { cwd: sourceDir });
	for (const path of paths) {
		const sourcePath = resolve(sourceDir, path);
		const destPath = resolve(sdkGenPath, path);

		try {
			await Deno.mkdir(dirname(destPath), { recursive: true });
		} catch (e) {
			if (!(e instanceof Deno.errors.AlreadyExists)) {
				throw e;
			}
		}

		let content = "";
		if (path.endsWith(".ts")) content += autoGenHeader("//") + "\n\n";
		content += await Deno.readTextFile(sourcePath);
		await Deno.writeTextFile(destPath, content);
	}

	// Create dirs for apis
	try {
		await Deno.mkdir(resolve(sdkGenPath, "modules"), { recursive: true });
	} catch (e) {
		if (!(e instanceof Deno.errors.AlreadyExists)) {
			throw e;
		}
	}
}

async function generateTsConfig(sdkGenPath: string) {
	await Deno.writeTextFile(
		resolve(sdkGenPath, "tsconfig.json"),
		JSON.stringify({
			"compilerOptions": {
				"declaration": true,
				"target": "es2022",
				"module": "commonjs",
				"moduleResolution": "node",
				"outDir": "dist",
				"typeRoots": [
					"node_modules/@types",
				],
			},
			"exclude": [
				"dist",
				"node_modules",
			],
		}),
	);
}

export async function generateBackendAndModules(project: Project, sdkGenPath: string) {
	const apiBuilder = new GeneratedCodeBuilder(resolve(sdkGenPath, "src", "index.ts"), 2, Lang.TypeScript);

	apiBuilder.append`
        import { Client } from "./client/client.ts";
        import { Configuration, ConfigurationOpts } from "./client/configuration.ts";
    `;

	const imports = apiBuilder.chunk;

	apiBuilder.append`
    /** Singleton class for Rivet API */
    export class Rivet {
        /** Configuration for interacting with Rivet. */
        public configuration: Configuration;

        /** Client used to connect to the backend. */
        private client: Client;
    `;
	const properties = apiBuilder.chunk;
	apiBuilder.append`
      constructor(opts?: ConfigurationOpts) {
		  this.configuration = new Configuration(opts);
          this.client = new Client(this.configuration);
    `;
	const constructProperties = apiBuilder.chunk;
	apiBuilder.append`
            }
        }
    `;

	for (const mod of project.modules.values()) {
		const moduleNamePascal = pascalify(mod.name);
		const className = `Rivet${moduleNamePascal}`;

		// Create module API class
		const moduleApiBuilder = new GeneratedCodeBuilder(
			resolve(sdkGenPath, "src", "modules", `${mod.name}.ts`),
			2,
			Lang.TypeScript,
		);

		// Generate module docs
		let moduleDocs = "";
		if (mod.config.name) {
			moduleDocs += mod.config.name;
			if (mod.config.description) {
				moduleDocs += "\n\n" + mod.config.description;
			}
		} else {
			moduleDocs += moduleNamePascal;
		}

		// Write class
		moduleApiBuilder.append`
			import { Client } from "../client/client.ts";

			${convertToDocComment(moduleDocs)}
			export class ${className} {
				client: Client;

				constructor(client: Client) {
					this.client = client;
				}
        `;
		const scripts = moduleApiBuilder.chunk;
		moduleApiBuilder.append`
			}
        `;

		const scriptNames = Array.from(mod.scripts.keys());
		const escapedScriptNames = escapeReservedWords(scriptNames);

		for (const [i, scriptName] of scriptNames.entries()) {
			const escapedScriptName = escapedScriptNames[i]!;
			const script = mod.scripts.get(scriptName)!;

			if (!script.config.public) continue;

			const path = `/modules/${mod.name}/scripts/${script.name}/call`;

			// Add script docs
			let scriptDocs = "";
			if (script.config.name) {
				scriptDocs += script.config.name + "\n";
				if (script.config.description) {
					scriptDocs += script.config.description + "\n";
				}
			} else {
				scriptDocs += scriptName;
			}

			scripts.append`
                ${convertToDocComment(scriptDocs)}
                ${camelify(escapedScriptName)}(body: any = {}): Promise<any> {
                    return this.client.buildRequest("${mod.name}.${script.name}", "POST", "${path}", body);
                }
            `;
		}

		await moduleApiBuilder.write();

		// Add module to main API class
		imports.append`import { ${className} } from "./modules/${mod.name}.ts";`;
		properties.append`${mod.name}: ${className};`;
		constructProperties.append`this.${mod.name} = new ${className}(this.client);`;
	}

	await apiBuilder.write();
}

function escapeReservedWords(wordsList: string[]) {
	const escaped = [];
	const usedNames = new Set<string>();

	for (let word of wordsList) {
		while (RESERVED_WORDS.includes(word) || usedNames.has(word)) {
			word = "call_" + word;
		}

		usedNames.add(word);
		escaped.push(word);
	}

	return escaped;
}

export async function generateModuleAddons(project: Project, sdkGenPath: string) {
	for (const module of project.modules.values()) {
		const sdkAddonsPath = resolve(module.path, "sdk_addons", "typescript");
		const files = await glob.glob("**/*.ts", { cwd: sdkAddonsPath });
		for (const file of files) {
			const bodyRaw = await Deno.readTextFile(resolve(sdkAddonsPath, file));
			const body = autoGenHeader("//") + "\n\n" + bodyRaw;
			await Deno.writeTextFile(resolve(sdkGenPath, "src", file), body);
		}
	}
}

function convertToDocComment(input: string): string {
	return `/**\n * ${input.split("\n").join("\n * ")}\n */`;
}
