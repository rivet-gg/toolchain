import { assertExists } from "@std/assert";
import { Client as PostgresClient } from "@bartlomieju/postgres";
import { Project } from "./project/project.ts";
import { verbose } from "./term/status.ts";
import { createOnce, getOrInitOnce } from "./utils/once.ts";
import { addShutdownHandler } from "./utils/shutdown_handler.ts";
import { InternalError } from "./error/mod.ts";

export function getDatabaseUrl(_project: Project): string {
	let dbUrl = Deno.env.get("DATABASE_URL")!;
	assertExists(dbUrl, "missing DATABASE_URL");
	return dbUrl;
}

/**
 * Default Postgres client to reuse between commands.
 */
const DEFAULT_CLIENT = createOnce<PostgresClient>();

export async function getClient(project: Project): Promise<PostgresClient> {
	return await getOrInitOnce(DEFAULT_CLIENT, async () => {
		const databaseUrl = getDatabaseUrl(project);

		const client = new PostgresClient(databaseUrl);

		await Promise.race([
			client.connect(),
			new Promise((_, reject) => {
				setTimeout(() => {
					reject(new InternalError("Database connection timed out"));
					client.end();
				}, 5000);
			}),
		]);

		addShutdownHandler(async () => {
			verbose("Shutting down default database client");
			await client.end();
		});

		return client;
	});
}
