import { parseArgs } from "@std/cli/parse-args";
import { executeCommand } from "./execute.ts";
import { commandSchema } from "./execute.ts";

const args = parseArgs(Deno.args);
const commandJson = args["command"];

if (!commandJson) {
	console.error("Missing --command argument");
	Deno.exit(1);
}

try {
	const command = JSON.parse(commandJson);
	const validatedCommand = commandSchema.parse(command);
	await executeCommand(validatedCommand);
} catch (error) {
	if (error instanceof SyntaxError) {
		console.error("Invalid JSON in --command argument");
	} else {
		console.error("Invalid command:", error);
	}
	Deno.exit(1);
}
