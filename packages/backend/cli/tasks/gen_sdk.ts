import { globalOptsSchema, initProject } from "../common.ts";
import { build, DbDriver, Format, Runtime } from "../../toolchain/build/mod.ts";
import { runTask } from "../task.ts";

runTask({
	inputSchema: globalOptsSchema,
	async run(input) {
		const project = await initProject(input);

		await build(project, {
			format: Format.Native,
			runtime: Runtime.Deno,
			dbDriver: DbDriver.NodePostgres,
			sdk: {},
			// Require schemas to be generated in order to generate SDk
			strictSchemas: true,
			skipDenoCheck: true,
		});
	},
});
