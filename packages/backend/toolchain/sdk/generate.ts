import { move } from "@std/fs";
import { resolve } from "@std/path";
import { CommandError, UnreachableError } from "../error/mod.ts";
import { Project } from "../project/mod.ts";
import { projectCachePath, SDK_PATH } from "../project/project.ts";
import { progress, success } from "../term/status.ts";

import { generateTypescript } from "./typescript/mod.ts";
import { generateUnity } from "./unity/mod.ts";
import { generateGodot } from "./godot/mod.ts";
import { SdkConfig } from "../config/project.ts";
import { unimplemented } from "@std/assert";

export enum SdkTarget {
	TypeScript = "typescript",
	Unity = "unity",
	Godot = "godot",
}

interface Generator {
}

export async function generateSdk(
	project: Project,
  sdk: SdkConfig
) {
	const targetString = targetToString(sdk.target);
	const sdkGenPath = resolve(projectCachePath(project, SDK_PATH), targetString);

	// Clear artifacts
	try {
		await Deno.remove(sdkGenPath, { recursive: true });
	} catch (err) {
		if (!(err instanceof Deno.errors.NotFound)) {
			throw err;
		}
	}

	progress("Building SDK", targetString);

	let sdkCopyPath = sdkGenPath;
	if (sdk.target == SdkTarget.TypeScript) {
		await generateTypescript(project, sdkGenPath);
	} else if (sdk.target == SdkTarget.Unity) {
		await generateUnity(project, sdkGenPath);
	} else if (sdk.target == SdkTarget.Godot) {
		await generateGodot(project, sdkGenPath);
	} else {
    throw new UnreachableError(sdk.target);
  }

	await move(sdkCopyPath, sdk.output, { overwrite: true });

	success("Success");
}

function targetToString(target: SdkTarget) {
	if (target == SdkTarget.TypeScript) return "typescript";
	if (target == SdkTarget.Unity) return "unity";
	if (target == SdkTarget.Godot) return "godot";
	throw new UnreachableError(target);
}
