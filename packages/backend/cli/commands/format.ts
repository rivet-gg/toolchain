import { z } from "zod";
import { globalOptsSchema, initProject } from "../common.ts";
import { listSourceFiles } from "../../toolchain/project/mod.ts";
import { UserError } from "../../toolchain/error/mod.ts";
import { denoExecutablePath } from "../../toolchain/utils/deno.ts";

export const optsSchema = globalOptsSchema.extend({
	check: z.boolean().nullable(),
});

type Opts = z.infer<typeof optsSchema>;

export async function execute(opts: Opts) {
	const project = await initProject(opts);

	const sourceFiles = await listSourceFiles(project, { localOnly: true });

	const cmd = await new Deno.Command(denoExecutablePath(), {
		args: [
			"fmt",
			...opts.check ? ["--check"] : [],
			...sourceFiles,
		],
		stdout: "inherit",
		stderr: "inherit",
	}).output();

	if (!cmd.success) {
		throw new UserError("Format failed.", { paths: sourceFiles });
	}
}
