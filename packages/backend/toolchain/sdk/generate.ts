import { exists, move } from "@std/fs";
import { resolve } from "@std/path";
import { UnreachableError } from "../error/mod.ts";
import { Project } from "../project/mod.ts";
import { projectDataPath, SDK_PATH } from "../project/project.ts";
import { progress, success } from "../term/status.ts";

import { generateTypescript } from "./typescript/mod.ts";
import { generateUnity } from "./unity/mod.ts";
import { generateGodot } from "./godot/mod.ts";
import { SdkConfig } from "../config/project.ts";

export enum SdkTarget {
	TypeScript = "typescript",
	Unity = "unity",
	Godot = "godot",
}

export async function generateSdk(
	project: Project,
	sdk: SdkConfig,
) {
	const targetString = targetToString(sdk.target);
	const sdkGenPath = resolve(projectDataPath(project, SDK_PATH), targetString);
  const sdkOutput = resolve(Deno.cwd(), sdk.output);

	// Clear artifacts
	try {
		await Deno.remove(sdkGenPath, { recursive: true });
	} catch (err) {
		if (!(err instanceof Deno.errors.NotFound)) {
			throw err;
		}
	}

	progress("Building SDK", targetString);

  // Generate files
  //
  // preservePaths is used for preserving generated artifacts from things like
  // `npm install`.
  let preservePaths: string[];
	if (sdk.target == SdkTarget.TypeScript) {
    preservePaths = ["dist"];
		await generateTypescript(project, sdkGenPath);
	} else if (sdk.target == SdkTarget.Unity) {
    preservePaths = [];
		await generateUnity(project, sdkGenPath);
	} else if (sdk.target == SdkTarget.Godot) {
    preservePaths = [];
		await generateGodot(project, sdkGenPath);
	} else {
		throw new UnreachableError(sdk.target);
	}

  // Delete target dir
  const preserveTempDir = await Deno.makeTempDir();
	if (await exists(sdkOutput, { isDirectory: true })) {
    // Preserve files before deleting the output path
    for (const path of preservePaths) {
      const srcPreservePath = resolve(sdkOutput, path);
      const dstPreservePath = resolve(preserveTempDir, path);
      if (await exists(srcPreservePath)) {
        await move(srcPreservePath, dstPreservePath);
      } else {
      }
    }

    // Remove output
		await Deno.remove(sdkOutput, { recursive: true });
	}

  // Move generated SDK
	await move(sdkGenPath, sdkOutput, { overwrite: true });

  // Move back preserved files
  for (const path of preservePaths) {
    const srcPreservePath = resolve(preserveTempDir, path);
    const dstPreservePath = resolve(sdkOutput, path);
    if (await exists(srcPreservePath, { isDirectory: true })) {
      await move(srcPreservePath, dstPreservePath, { overwrite: false });
    }
  }

	success("Success");
}

function targetToString(target: SdkTarget) {
	if (target == SdkTarget.TypeScript) return "typescript";
	if (target == SdkTarget.Unity) return "unity";
	if (target == SdkTarget.Godot) return "godot";
	throw new UnreachableError(target);
}
