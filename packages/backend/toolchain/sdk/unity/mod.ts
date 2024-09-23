// unity/mod.ts

import { pascalify } from "../../../case_conversion/mod.ts";
import { dirname, resolve } from "@std/path";
import { GeneratedCodeBuilder, Lang } from "../../build/gen/code_builder.ts";
import { Project } from "../../project/mod.ts";
import { autoGenHeader } from "../../build/misc.ts";

// C# reserved words
const RESERVED_WORDS = [
	"abstract",
	"as",
	"base",
	"bool",
	"break",
	"byte",
	"case",
	"catch",
	"char",
	"checked",
	"class",
	"const",
	"continue",
	"decimal",
	"default",
	"delegate",
	"do",
	"double",
	"else",
	"enum",
	"event",
	"explicit",
	"extern",
	"false",
	"finally",
	"fixed",
	"float",
	"for",
	"foreach",
	"goto",
	"if",
	"implicit",
	"in",
	"int",
	"interface",
	"internal",
	"is",
	"lock",
	"long",
	"namespace",
	"new",
	"null",
	"object",
	"operator",
	"out",
	"override",
	"params",
	"private",
	"protected",
	"public",
	"readonly",
	"ref",
	"return",
	"sbyte",
	"sealed",
	"short",
	"sizeof",
	"stackalloc",
	"static",
	"string",
	"struct",
	"switch",
	"this",
	"throw",
	"true",
	"try",
	"typeof",
	"uint",
	"ulong",
	"unchecked",
	"unsafe",
	"ushort",
	"using",
	"virtual",
	"void",
	"volatile",
	"while",
];

export async function generateUnity(project: Project, sdkGenPath: string) {
	await generateBackendAndModules(project, sdkGenPath);
}

export async function generateBackendAndModules(project: Project, sdkGenPath: string) {
	const apiBuilder = new GeneratedCodeBuilder(resolve(sdkGenPath, "Rivet.cs"), 2, Lang.CSharp);

	apiBuilder.append`
    using System;
    using System.Collections.Generic;
    using Newtonsoft.Json;

    namespace Rivet
    {
      public class RivetSingleton
      {
        // API for interacting with modules.

        // Client used to connect to the backend.
        public Client Client { get; private set; }

        // Configuration for how to connect to the backend.
        public Configuration Configuration { get; private set; }

        public RivetSingleton()
        {
          this.Configuration = new Configuration();
          this.Client = new Client(this.Configuration);
          this.InitModules();
        }
  `;

	const modulesCode = apiBuilder.chunk;

	for (const mod of project.modules.values()) {
		const moduleNamePascal = pascalify(mod.name);
		const className = `Rivet${moduleNamePascal}`;

		// Create module API class
		const moduleApiBuilder = new GeneratedCodeBuilder(
			resolve(sdkGenPath, "Modules", `${moduleNamePascal}.cs`),
			2,
			Lang.CSharp,
		);

		// Add module documentation
		let moduleDocs = "";
		if (mod.config.name) {
			moduleDocs += `/// <summary>\n    /// ${mod.config.name}`;
			if (mod.config.description) {
				moduleDocs += `\n    /// ${mod.config.description}`;
			}
			moduleDocs += "\n    /// </summary>";
		} else {
			moduleDocs += `/// <summary>\n    /// ${moduleNamePascal}\n    /// </summary>`;
		}

		moduleApiBuilder.append`
      using System;
      using System.Collections.Generic;
      using Newtonsoft.Json;

      namespace Rivet.Modules.${moduleNamePascal}
      {
        ${moduleDocs}
        public class ${className}
        {
          private Client _client;

          public ${className}(Client client)
          {
            this._client = client;
          }
    `;

		// Generate methods for each script
		const scriptsCode = moduleApiBuilder.chunk;
		const scriptNames = Array.from(mod.scripts.keys());
		const escapedScriptNames = escapeReservedWords(scriptNames);

		for (const [i, scriptName] of scriptNames.entries()) {
			const escapedScriptName = escapedScriptNames[i];
			const script = mod.scripts.get(scriptName)!;

			if (!script.config.public) continue;

			const path = `/modules/${mod.name}/scripts/${script.name}/call`;

			// Add script documentation
			let scriptDocs = "";
			if (script.config.name) {
				scriptDocs += `/// <summary>\n        /// ${script.config.name}`;
				if (script.config.description) {
					scriptDocs += `\n        /// ${script.config.description}`;
				}
				scriptDocs += "\n        /// </summary>";
			} else {
				scriptDocs += `/// <summary>\n        /// ${scriptName}\n        /// </summary>`;
			}

			scriptsCode.append`
        ${scriptDocs}
        public Request ${escapedScriptName}(Dictionary<string, object> body = null)
        {
          return this._client.BuildRequest("POST", "${path}", body);
        }
      `;
		}

		moduleApiBuilder.append`
        }
      }
    `;

		await moduleApiBuilder.write();

		// Add module to main API class
		modulesCode.append`
      public ${className} ${moduleNamePascal} { get; private set; }
    `;
	}

	// Initialize modules in the constructor
	modulesCode.append`
      private void InitModules()
      {
  `;

	for (const mod of project.modules.values()) {
		const moduleNamePascal = pascalify(mod.name);
		const className = `Rivet${moduleNamePascal}`;
		modulesCode.append`
        this.${moduleNamePascal} = new ${className}(this.Client);
    `;
	}

	modulesCode.append`
      }
  `;

	// Close the RivetSingleton class and namespace
	apiBuilder.append`
      }
    }
  `;

	await apiBuilder.write();
}

function escapeReservedWords(wordsList: string[]) {
	const escaped = [];
	const usedNames = new Set();

	for (let [i, word] of wordsList.entries()) {
		while (RESERVED_WORDS.includes(word) || usedNames.has(word)) {
			word = "_" + word;
		}

		usedNames.add(word);
		escaped[i] = word;
	}

	return escaped;
}

// // Assuming you have a function to get the schema definitions
// function generateCSharpClassFromSchema(schemaName: string, schema: any): string {
// 	let classCode = `public class ${schemaName}\n{\n`;
// 	for (const [propName, propType] of Object.entries(schema.properties)) {
// 	  // Map Zod types to C# types
// 	  const csharpType = mapZodTypeToCSharp(propType);
// 	  classCode += `    [JsonProperty("${propName}")]\n`;
// 	  classCode += `    public ${csharpType} ${pascalify(propName)} { get; set; }\n\n`;
// 	}
// 	classCode += `}\n`;
// 	return classCode;
//   }
//
//   function mapZodTypeToCSharp(zodType: any): string {
// 	// Implement type mapping from Zod types to C# types
// 	// For example:
// 	if (zodType === 'string') return 'string';
// 	if (zodType === 'number') return 'int'; // Adjust based on actual type
// 	// Add other type mappings
// 	return 'object'; // Default to object
//   }
