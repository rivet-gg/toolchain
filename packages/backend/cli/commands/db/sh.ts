import { z } from "zod";
import { assertExists } from "@std/assert";
import { UserError } from "../../../toolchain/error/mod.ts";
import { openShell } from "../../../toolchain/postgres/manager.ts";
import { DEFAULT_DATABASE, ensurePostgresRunning, getDefaultPostgresManager } from "../../../toolchain/postgres/mod.ts";
import { globalOptsSchema, initProject } from "../../common.ts";

export const optsSchema = globalOptsSchema;

type Opts = z.infer<typeof optsSchema>;

export async function execute(opts: Opts) {
	// Validate terminal
	if (!Deno.stdin.isTerminal()) {
		throw new UserError("Cannot run this command without a terminal.", {
			suggest:
				"This is likely because you're running from a non-interactive shell, such as a CI environment. Run this command in a terminal that supports TTY.",
		});
	}

	const project = await initProject(opts);
	await ensurePostgresRunning(project);
	const manager = await getDefaultPostgresManager(project);
	assertExists(manager);
	await openShell(manager, DEFAULT_DATABASE);
}
