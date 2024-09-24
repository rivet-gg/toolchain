import { z } from "zod";
import { globalOptsSchema, initProject } from "../common.ts";
import { generateOpenApi } from "../../toolchain/build/openapi.ts";
import { build, DbDriver, Format, Runtime } from "../../toolchain/build/mod.ts";
import { runTask } from "../task.ts";

export const inputSchema = globalOptsSchema.extend({
	output: z.string(),
});

runTask({
	inputSchema,
	async run(input) {
		const project = await initProject(input);

		await build(project, {
			format: Format.Native,
			runtime: Runtime.Deno,
			dbDriver: DbDriver.NodePostgres,
			// Require schemas to be generated in order to build OpenAPI types
			strictSchemas: true,
			skipDenoCheck: true,
		});

		await generateOpenApi(project, input.output);
	},
});
