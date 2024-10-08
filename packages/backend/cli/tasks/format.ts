import { z } from "zod";
import { globalOptsSchema, initProject } from "../common.ts";
import { DENO_JSON_PATH, DENO_LOCK_PATH, listSourceFiles, projectDataPath } from "../../toolchain/project/mod.ts";
import { UserError } from "../../toolchain/error/mod.ts";
import { denoExecutablePath } from "../../toolchain/utils/deno.ts";
import { runTask } from "../task.ts";

export const inputSchema = globalOptsSchema.extend({
	check: z.boolean().nullable(),
});

runTask({
	inputSchema,
	async run(input) {
		const project = await initProject(input);

		const sourceFiles = await listSourceFiles(project, { localOnly: true });

		const cmd = await new Deno.Command(denoExecutablePath(), {
			args: [
				"fmt",
				"--config",
				projectDataPath(project, DENO_JSON_PATH),
				"--lock",
				projectDataPath(project, DENO_LOCK_PATH),
				...input.check ? ["--check"] : [],
				...sourceFiles,
			],
			stdout: "inherit",
			stderr: "inherit",
		}).output();

		if (!cmd.success) {
			throw new UserError("Format failed.", { paths: sourceFiles });
		}
	},
});
