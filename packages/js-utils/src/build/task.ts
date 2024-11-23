import { z } from "zod";
import { globalOptsSchema } from "../task/common.ts";
import { runTask } from "../task/task.ts";
import { build } from "./build.ts";

export const inputSchema = z.object({}).merge(globalOptsSchema);

runTask({
	inputSchema,
	async run(input) {
		await build();
	},
});
