#!/usr/bin/env -S deno run --allow-read --allow-write --allow-env

import { assertExists } from "jsr:@std/assert";
import { emptyDir, copy } from "jsr:@std/fs";
import { join } from "jsr:@std/path";

const sdksDir = "./sdks";
const rustSdkDir = join(sdksDir, "rust");
const cargoTomlPath = join(rustSdkDir, "Cargo.toml");

await emptyDir(rustSdkDir);

const eeRepoPath = Deno.env.get("EE_REPO_PATH");
assertExists(eeRepoPath, "EE_REPO_PATH environment variable is not set");
await copy(join(eeRepoPath, "sdks", "full", "rust"), rustSdkDir, { overwrite: true });

let cargoToml = await Deno.readTextFile(cargoTomlPath);
cargoToml = cargoToml.replace(
  /\[dependencies\.reqwest\]/,
  "[dependencies.reqwest]\ndefault-features = false"
);
await Deno.writeTextFile(cargoTomlPath, cargoToml);

