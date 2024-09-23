import { verbose } from "../term/status.ts";

export type ShutdownFn = () => Promise<void>;

interface ShutdownHandler {
	fns: ShutdownFn[];
	abortControllers: AbortController[];
}

const SHUTDOWN_HANDLER: ShutdownHandler = {
	fns: [],
	abortControllers: [],
};

/**
 * Adds a function to be ran on shutdown.
 */
export function addShutdownHandler(fn: ShutdownFn) {
	SHUTDOWN_HANDLER.fns.push(fn);
}

/**
 * Creates a new abort controller that will be destroyed on shutdown.
 *
 * This abort controller can be manually triggered by other functions also.
 */
export function createShutdownAbortController(): AbortController {
	const controller = new AbortController();
	SHUTDOWN_HANDLER.abortControllers.push(controller);
	return controller;
}

/**
 * Runs all shutdown functions.
 */
export async function runShutdown() {
	verbose(`Aborting ${SHUTDOWN_HANDLER.abortControllers.length} abort controllers`);
	SHUTDOWN_HANDLER.abortControllers.forEach((x) => x.abort("Shutdown"));

	verbose(`Running ${SHUTDOWN_HANDLER.fns.length} shutdown handlers`);
	await Promise.all(SHUTDOWN_HANDLER.fns.map((fn) => fn()));
}
