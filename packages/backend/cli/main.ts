import { parseArgs } from "@std/cli/parse-args";
import { executeCommand } from "./execute.ts";
import { commandSchema } from "./execute.ts";
import { runShutdown } from "../toolchain/utils/shutdown_handler.ts";
import { printError, UserError } from "../toolchain/error/mod.ts";

let exitCode = 0;
try {
	// Parse flags
	const args = parseArgs(Deno.args);
	const commandJson = args["command"];
	if (!commandJson) {
		throw new UserError("Missing --command argument");
	}

	// Parse coman
	let command;
	try {
		command = JSON.parse(commandJson);
	} catch (cause) {
		throw new UserError("Invalid command JSON", { cause });
	}

	// Validate command
	let validatedCommand = commandSchema.safeParse(command);
	if (!validatedCommand.success) {
		throw new UserError("Command violates schema", { details: JSON.stringify(validatedCommand.error, null, 2) });
	}

	// Execute command
	await executeCommand(validatedCommand.data);
} catch (err) {
	printError(err);
	exitCode = 1;
} finally {
	runShutdown();
}

Deno.exit(exitCode);
