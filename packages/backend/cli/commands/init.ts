import { z } from "zod";
import { globalOptsSchema } from "../common.ts";
import { templateProject } from "../../toolchain/template/project.ts";

export const optsSchema = z.object({
	dir: z.string().default("."),
}).merge(globalOptsSchema);

type Opts = z.infer<typeof optsSchema>;

export async function execute(opts: Opts) {
	await templateProject(opts.dir);

	console.log("Welcome to Rivet");
	console.log("");
	console.log("Created rivet.json");
	console.log("");
	console.log("Get started at https://rivet.gg/docs");
}
