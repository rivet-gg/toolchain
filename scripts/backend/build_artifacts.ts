#!/usr/bin/env -S deno run -A

import { DRIZZLE_ORM_PACKAGE } from "../../packages/backend/toolchain/drizzle_consts.ts";
import { resolve } from "@std/path";
import { copy, exists } from "@std/fs";

// Hack to allow Yarn to work on Windows
const YARN_COMMAND = Deno.build.os === "windows" ? "cmd" : "yarn";
const YARN_ARGS = Deno.build.os === "windows" ? ["/c", "yarn"] : [];

/**
 * Path to the root of the repo. Used for reading & writing files to the
 * project.
 */
export function projectRoot() {
  const dirname = import.meta.dirname;
  if (!dirname) throw new Error("Missing dirname");

  return resolve(dirname, "..", "..", "packages", "backend");
}

async function getPackageExports(
  moduleName: any,
  excludedSymbols: string[],
): Promise<string[]> {
  const module = await import(moduleName);
  const symbols = Object.keys(module).filter(
    (key) => !excludedSymbols.includes(key),
  );
  return symbols;
}

async function generateDrizzleOrmArtifacts() {
  const exports = {
    drizzleOrm: await getPackageExports(DRIZZLE_ORM_PACKAGE, [
      "BaseName",
      "Columns",
      "ExtraConfigBuilder",
      "ExtraConfigColumns",
      "IsAlias",
      "OriginalName",
      "Schema",
      "TableName",
      "applyMixins",
      "getTableLikeName",
      "mapResultRow",
      "mapUpdateSet",
      "orderSelectedFields",
    ]),
    drizzleOrmPgCore: await getPackageExports(
      `${DRIZZLE_ORM_PACKAGE}/pg-core`,
      [
        "InlineForeignKeys",
        "pgEnumWithSchema",
        "pgMaterializedViewWithSchema",
        "pgSequenceWithSchema",
        "pgTableWithSchema",
        "pgViewWithSchema",
      ],
    ),
  };

  const outputPath = resolve(projectRoot(), "artifacts", "drizzle_orm.json");
  await Deno.writeTextFile(
    outputPath,
    JSON.stringify({ exports }),
  );
  console.log(`[drizzle] Wrote Drizzle ORM exports to ${outputPath}`);
}

async function buildEditor() {
  const EDITOR_PATH = resolve(import.meta.dirname!, "..", "..", "packages", "editor");

  const editorOutDir = await Deno.makeTempDir();

  console.log(`[editor] Installing packages`)
  const installResult = await new Deno.Command(YARN_COMMAND, {
    args: YARN_ARGS,
    cwd: EDITOR_PATH,
    env: {
      "NODE_ENV": "production",
    },
    stdout: "inherit",
    stderr: "inherit",
  }).output();

  if (!installResult.success) {
    throw new Error(`Failed to install packages`);
  }

  console.log(`[editor] Building`)
  const buildResult = await new Deno.Command(YARN_COMMAND, {
    args: [
      ...YARN_ARGS,
      "build",
    ],
    cwd: EDITOR_PATH,
    env: {
      "NODE_ENV": "production",
      "TURBO_UI": "0",
      "VITE_OUT_DIR": editorOutDir,
    },
    stdout: "inherit",
    stderr: "inherit",
  }).output();

  if (!buildResult.success) {
    throw new Error(`Failed to build editor`);
  }

  console.log(`[editor] Copying to artifacts`)
  const artifactsPath = resolve(projectRoot(), "artifacts", "editor");
  if (await exists(artifactsPath, { isDirectory: true })) {
	  await Deno.remove(artifactsPath, { recursive: true });
  }
  await copy(editorOutDir, artifactsPath);
}

async function main() {
	await Deno.mkdir(resolve(projectRoot(), "artifacts"), { recursive: true });
    await generateDrizzleOrmArtifacts();
    await buildEditor();
}

main();
