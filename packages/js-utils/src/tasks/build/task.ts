import { z } from "zod";
import { globalOptsSchema } from "../../util/task/common.ts";
import { runTask } from "../../util/task/task.ts";
import { build } from "./build.ts";

export const inputSchema = z.object({
  minify: z.boolean(),
  analyzeResult: z.boolean(),
}).merge(globalOptsSchema);

export type Input = z.infer<typeof inputSchema>;

runTask({
	inputSchema,
	async run(input) {
		await build(input);
	},
});
