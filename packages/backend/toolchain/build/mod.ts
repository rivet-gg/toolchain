import { Project } from "../project/project.ts";
import { createBuildState, waitForBuildPromises } from "../build_state/mod.ts";
import { success } from "../term/status.ts";
import { planProjectBuild } from "./plan/project.ts";
import { UnreachableError } from "../error/mod.ts";
import { ensureLocked } from "../project/mod.ts";

/**
 * Which format to use for building.
 */
export enum Format {
	Native = "native",
	Bundled = "bundled",
}

/**
 * Which runtime to target when building.
 */
export enum Runtime {
	Deno = "deno",
	CloudflareWorkersPlatforms = "cloudflare_workers_platforms",
}

/**
 * Which DB driver to use for the runtime.
 */
export enum DbDriver {
	NodePostgres = "node_postgres",
	NeonServerless = "neon_serverless",
	CloudflareHyperdrive = "cloudflare_hyperdrive",
}

export enum MigrateMode {
	Dev = "dev",
	Generate = "generate",
	GenerateAndApply = "generate_and_apply",
	Apply = "apply",
}

/**
 * Stores options used in the build command.
 */
export interface BuildOpts {
	format: Format;
	runtime: Runtime;
	dbDriver: DbDriver;
	/** If true, parse TypeScript to generate JSON schemas to be validated at runtime. */
	strictSchemas: boolean;
	/** If true, don't run `deno check` on the generated code. */
	skipDenoCheck: boolean;
	/** If true, generate the SDK. */
	sdk?: Record<never, never>;
	/** If exists, run database migrations. */
	migrate?: {
		mode: MigrateMode;
	};
	signal?: AbortSignal;
}

export async function build(project: Project, opts: BuildOpts) {
	opts.signal?.throwIfAborted();
	ensureLocked(project);

	const buildState = await createBuildState(project, opts.signal);

	await planProjectBuild(buildState, project, opts);

	// Wait for any remaining build steps
	await waitForBuildPromises(buildState);

	success("Success");
}

export function runtimeToString(runtime: Runtime) {
	if (runtime == Runtime.Deno) return "Deno";
	if (runtime == Runtime.CloudflareWorkersPlatforms) return "Cloudflare";

	throw new UnreachableError(runtime);
}
