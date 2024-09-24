import { z } from "zod";
import { templateActor } from "../../../toolchain/template/actor.ts";
import { Casing } from "../../../toolchain/types/identifiers/defs.ts";
import { validateIdentifier } from "../../../toolchain/types/identifiers/mod.ts";
import { globalOptsSchema, initProject } from "../../common.ts";
import { runTask } from "../../task.ts";

export const inputSchema = z.object({
	module: z.string(),
	actor: z.string(),
}).merge(globalOptsSchema);

runTask({
	inputSchema,
	async run(input) {
		validateIdentifier(input.module, Casing.Snake);
		validateIdentifier(input.actor, Casing.Snake);

		await templateActor(await initProject(input), input.module, input.actor);
	},
});
