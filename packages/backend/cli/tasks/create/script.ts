import { z } from "zod";
import { templateScript } from "../../../toolchain/template/script.ts";
import { Casing } from "../../../toolchain/types/identifiers/defs.ts";
import { validateIdentifier } from "../../../toolchain/types/identifiers/mod.ts";
import { globalOptsSchema, initProject } from "../../common.ts";
import { runTask } from "../../task.ts";

export const inputSchema = z.object({
	module: z.string(),
	script: z.string(),
}).merge(globalOptsSchema);

runTask({
	inputSchema,
	async run(input) {
		validateIdentifier(input.module, Casing.Snake);
		validateIdentifier(input.script, Casing.Snake);

		await templateScript(await initProject(input), input.module, input.script);
	},
});
