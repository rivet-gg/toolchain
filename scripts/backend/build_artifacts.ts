#!/usr/bin/env -S deno run -A

import { DRIZZLE_ORM_PACKAGE } from "../../packages/backend/toolchain/drizzle_consts.ts";
import { resolve } from "@std/path";
import * as glob from "glob";

/**
 * Path to the root of the repo. Used for reading & writing files to the
 * project.
 */
export function projectRoot() {
  const dirname = import.meta.dirname;
  if (!dirname) throw new Error("Missing dirname");

  return resolve(dirname, "..", "..", "packages", "backend");
}

/**
 * @param files Files to include in the archive. Usually generated using glob.
 * @param outputPath JSON file to write the archive to.
 */
export async function buildArtifacts(
  { logName, rootPath, patterns, outputPath, globOpts, encode }: {
    logName: string;
    rootPath: string;
    patterns: string[];
    outputPath: string;
    globOpts?: any;
    encode?: "base64" | "string";
  },
) {
  encode = encode ?? "base64";

  // Glob files
  const files = await glob.glob(patterns, {
    cwd: rootPath,
    nodir: true,
    ...globOpts,
  });
  files.sort();

  // Build object
  //
  // Do this manually instead of with JSON.stringify in order to ensure there's
  // one entry per line in alphabetical order for more useful Git diffs.
  const archiveFileSizes: Record<string, number> = {};
  let archiveFile = "{\n";
  for (const [i, file] of files.entries()) {
    // Get file contents
    let content: string;
    if (encode == "base64") {
      const data = await Deno.readFile(resolve(rootPath, file));
      content = btoa(
        new Uint8Array(data).reduce(
          (acc, byte) => acc + String.fromCharCode(byte),
          "",
        ),
      );
    } else if (encode == "string") {
      content = await Deno.readTextFile(resolve(rootPath, file));
    } else {
      throw new Error("unreachable");
    }

    // Add to JSON
    archiveFile += `${JSON.stringify(file)}:${JSON.stringify(content)}`;
    if (i != files.length - 1) archiveFile += ",\n";
    else archiveFile += "\n";

    archiveFileSizes[file] = content.length;
  }
  archiveFile += "}";

  // Write schema to file
  await Deno.writeTextFile(
    outputPath,
    archiveFile,
  );

  console.log(`[${logName}] Wrote archive to ${outputPath}`);

  // Print largest files. File sizes are in encoded format.
  console.log(`[${logName}] Largest files:`);
  Object.keys(archiveFileSizes)
    .sort((a, b) => archiveFileSizes[b]! - archiveFileSizes[a]!).slice(0, 10)
    .forEach((key) =>
      console.log(
        `[${logName}]`,
        key,
        Math.ceil(archiveFileSizes[key]! / 1000) + "KB",
      )
    );
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

async function buildDrizzleArtifacts() {
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

async function buildDynamicArchive() {
  const outputPath = resolve(
    projectRoot(),
    "artifacts",
    "dynamic_archive.json",
  );
  await buildArtifacts({
    logName: "dynamic",
    rootPath: resolve(projectRoot(), "dynamic"),
    patterns: ["**/*.{ts,gd,cfg}"],
    outputPath,
    encode: "string",
  });
  console.log(`[dynamic] Wrote dynamic archive to ${outputPath}`);
}

async function buildRuntimeArchive() {
  const outputPath = resolve(
    projectRoot(),
    "artifacts",
    "packages_archive.json",
  );
  await buildArtifacts({
    logName: "packages",
    rootPath: projectRoot(),
    patterns: [
      "{runtime,case_conversion,path_resolver}/**/*.ts",
    ],
    outputPath,
    encode: "string",
  });
  console.log(`[packages] Wrote runtime archive to ${outputPath}`);
}

async function buildEditor() {
  const EDITOR_PATH = resolve(import.meta.dirname!, "..", "..", "packages", "editor");

  const editorOutDir = await Deno.makeTempDir();

  console.log(`[editor] Installing packages`)
  const installResult = await new Deno.Command("yarn", {
    cwd: EDITOR_PATH,
    env: {
      "NODE_ENV": "production",
    },
    stdout: "inherit",
    stderr: "inherit",
  }).output();

  if (!installResult.success) {
    throw new Error(`Failed to install packages: ${new TextDecoder().decode(installResult.stderr)}`);
  }

  console.log(`[editor] Building`)
  const buildResult = await new Deno.Command("yarn", {
    args: [
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
    throw new Error(`Failed to build editor: ${new TextDecoder().decode(buildResult.stderr)}`);
  }

  await buildArtifacts({
    logName: "editor",
    rootPath: editorOutDir,
    patterns: [
      "**",
    ],
    outputPath: resolve(projectRoot(), "artifacts", "editor_archive.json"),
    encode: "base64",
  });

  console.log(`[editor] Cleaning up editor out dir`)
  await Deno.remove(editorOutDir, { recursive: true });
}

async function main() {
    await buildDrizzleArtifacts();
    await buildDynamicArchive();
    await buildRuntimeArchive();
    await buildEditor();
}

main();
