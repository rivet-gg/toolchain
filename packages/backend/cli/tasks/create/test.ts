import { z } from "zod";
import { templateTest } from "../../../toolchain/template/test.ts";
import { Casing } from "../../../toolchain/types/identifiers/defs.ts";
import { validateIdentifier } from "../../../toolchain/types/identifiers/mod.ts";
import { globalOptsSchema, initProject } from "../../common.ts";
import { runTask } from "../../task.ts";

export const inputSchema = z.object({
	module: z.string(),
	test: z.string(),
}).merge(globalOptsSchema);

runTask({
	inputSchema,
	async run(input) {
		validateIdentifier(input.module, Casing.Snake);
		validateIdentifier(input.test, Casing.Snake);

		await templateTest(await initProject(input), input.module, input.test);
	},
});
