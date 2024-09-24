import { z } from "zod";
import { globalOptsSchema } from "../common.ts";
import { templateProject } from "../../toolchain/template/project.ts";
import { runTask } from "../task.ts";

export const inputSchema = z.object({
	dir: z.string().default("."),
}).merge(globalOptsSchema);

runTask({
  inputSchema,
  async run(input) {
	await templateProject(input.dir);

	console.log("Welcome to Rivet");
	console.log("");
	console.log("Created rivet.json");
	console.log("");
	console.log("Get started at https://rivet.gg/docs");

  }
})
