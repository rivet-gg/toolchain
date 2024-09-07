import { z } from "zod";
import { globalOptsSchema, initProject } from "../../../common.ts";
import { getDefaultPostgresManager } from "../../../../toolchain/postgres/mod.ts";
import { setup } from "../../../../toolchain/postgres/manager.ts";
import { success, warn } from "../../../../toolchain/term/status.ts";

export const optsSchema = globalOptsSchema;

type Opts = z.infer<typeof optsSchema>;

export async function execute(opts: Opts) {
	const project = await initProject(opts);
	const manager = await getDefaultPostgresManager(project);
	if (manager) {
		await setup(manager);
		success("Postgres instance started");
	} else {
		warn("Postgres is disabled");
	}
}
