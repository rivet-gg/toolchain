import { z } from "zod";
import { globalOptsSchema } from "../../util/task/common.ts";
import { runTask } from "../../util/task/task.ts";
import { build } from "./build.ts";

export const inputSchema = z.object({
	entryPoint: z.string(),
	outDir: z.string(),
	deno: z.object({
		configPath: z.string().optional(),
		importMapUrl: z.string().optional(),
		lockPath: z.string().optional(),
	}),
	bundle: z.object({
		minify: z.boolean(),
		analyzeResult: z.boolean(),
		logLevel: z.string(),
	}),
}).merge(globalOptsSchema);

export type Input = z.infer<typeof inputSchema>;

export interface Output {
	files: string[];
	analyzedMetafile?: string;
}

runTask({
	inputSchema,
	async run(input) {
		let output = await build(input);
		console.log(JSON.stringify(output));
	},
});
