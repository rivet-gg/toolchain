import { z } from "zod";
import { globalOptsSchema, initProject } from "../common.ts";
import { listSourceFiles } from "../../toolchain/project/mod.ts";
import { UserError } from "../../toolchain/error/mod.ts";
import { denoExecutablePath } from "../../toolchain/utils/deno.ts";
import { runTask } from "../task.ts";

export const inputSchema = z.object({
	// Add any command-specific options here
}).merge(globalOptsSchema);

runTask({
  inputSchema,
  async run(input) {

	const project = await initProject(input);

	const sourceFiles = await listSourceFiles(project, { localOnly: true });

	const cmd = await new Deno.Command(denoExecutablePath(), {
		args: [
			"lint",
			...sourceFiles,
		],
		stdout: "inherit",
		stderr: "inherit",
	}).output();

	if (!cmd.success) {
		throw new UserError("Lint failed.", { paths: sourceFiles });
	}
  }
})
