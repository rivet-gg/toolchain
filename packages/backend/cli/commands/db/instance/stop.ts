import { z } from "zod";
import { globalOptsSchema, initProject } from "../../../common.ts";
import { getDefaultPostgresManager } from "../../../../toolchain/postgres/mod.ts";
import { Status, status } from "../../../../toolchain/postgres/manager.ts";
import { info, warn } from "../../../../toolchain/term/status.ts";
import { UnreachableError } from "../../../../toolchain/error/mod.ts";

export const optsSchema = globalOptsSchema;

type Opts = z.infer<typeof optsSchema>;

export async function execute(opts: Opts) {
	const project = await initProject(opts);
	const manager = await getDefaultPostgresManager(project);
	if (manager) {
		let statusText: string;
		const currentStatus = await status(manager);
		if (currentStatus === Status.NotInstalled) {
			statusText = "Not installed";
		} else if (currentStatus === Status.Installed) {
			statusText = "Installed";
		} else if (currentStatus === Status.Initialized) {
			statusText = "Initialized";
		} else if (currentStatus === Status.DefaultDatabaseNotCreated) {
			statusText = "Default database not created";
		} else if (currentStatus === Status.Stopped) {
			statusText = "Stopped";
		} else if (currentStatus === Status.Started) {
			statusText = "Started";
		} else {
			throw new UnreachableError(currentStatus);
		}
		info("Status", statusText);
	} else {
		warn("Postgres is disabled");
	}
}
