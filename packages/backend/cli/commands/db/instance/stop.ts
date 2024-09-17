import { z } from "zod";
import { globalOptsSchema, initProject } from "../../../common.ts";
import { getDefaultPostgresManager } from "../../../../toolchain/postgres/mod.ts";
import { stop } from "../../../../toolchain/postgres/manager.ts";
import { success, warn } from "../../../../toolchain/term/status.ts";

export const optsSchema = globalOptsSchema;

type Opts = z.infer<typeof optsSchema>;

export async function execute(opts: Opts) {
	const project = await initProject(opts);
	const manager = await getDefaultPostgresManager(project);
	if (manager) {
		await stop(manager);
		success("Postgres instance stopped");
	} else {
		warn("Postgres is disabled");
	}
}
