#!/usr/bin/env -S deno run -A

import { resolve } from "jsr:@std/path";
import { assert } from "jsr:@std/assert";
import { S3Bucket } from "https://deno.land/x/s3@0.5.0/mod.ts";
import { buildCross } from "../build/build_cross.ts";

function getRequiredEnvVar(name: string): string {
    const value = Deno.env.get(name);
    if (!value) {
        throw new Error(`Required environment variable ${name} is not set`);
    }
    return value;
}

const toolchainVersion = getRequiredEnvVar("TOOLCHAIN_VERSION");
const awsAccessKeyId = getRequiredEnvVar("AWS_ACCESS_KEY_ID");
const awsSecretAccessKey = getRequiredEnvVar("AWS_SECRET_ACCESS_KEY");

assert(/^v(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$/.test(toolchainVersion), "ASSET_VERSION must be a valid semantic version starting with 'v'");

const OUTPUT_DIR = Deno.env.get("OUTPUT_DIR") ?? await Deno.makeTempDir({ prefix: "toolchain-" });
console.log("Work dir:", OUTPUT_DIR);
const DIST_DIR = resolve(OUTPUT_DIR, "dist");

async function buildCrossPlatform() {
    // We build this in the repo dir in order to make sure we use the build cache
    console.log("Building cross-platform binaries");
    try {
      await buildCross(DIST_DIR);
    } catch (err) {
      throw new Error(`Failed to build cross-platform binaries: ${err}`);
    }
}

async function generateZipFiles(): Promise<string[]> {
    console.log("Generating zip files");
    const zipPaths: string[] = [];
    for await (const entry of Deno.readDir(DIST_DIR)) {
        if (entry.isDirectory) {
            const folderName = entry.name;
            const folderPath = resolve(DIST_DIR, folderName);
            for await (const file of Deno.readDir(folderPath)) {
                if (file.isFile) {
                    const fileName = file.name;
                    const zipPath = resolve(OUTPUT_DIR, `${folderName}-${fileName}.zip`);
                    const zipOutput = await (new Deno.Command("zip", {
                        args: ["-j", zipPath, resolve(folderPath, fileName)],
                        stdout: "inherit",
                        stderr: "inherit",
                    })).output();
                    assert(zipOutput.success, `Failed to create zip for ${folderName}/${fileName}`);
                    console.log(`Zip file created: ${zipPath}`);
                    zipPaths.push(zipPath);
                }
            }
        }
    }
    return zipPaths;
}

async function uploadZipsToS3(zipPaths: string[]): Promise<{ zipUrls: string[] }> {
    console.log("Uploading zip files to S3");
    const bucket = new S3Bucket({
        accessKeyID: awsAccessKeyId,
        secretKey: awsSecretAccessKey,
        bucket: "rivet-releases",
        region: "auto",
        endpointURL: "https://2a94c6a0ced8d35ea63cddc86c2681e7.r2.cloudflarestorage.com/rivet-releases",
    });

    const zipUrls: string[] = [];

    for (const zipPath of zipPaths) {
        const fileName = zipPath.split("/").pop()!;
        const [folderName, ...rest] = fileName.split("-");
        const zipObjectKey = `toolchain/${toolchainVersion}/${folderName}/${rest.join("-")}`;

        const zipFileData = await Deno.readFile(zipPath);

        await bucket.putObject(zipObjectKey, zipFileData);

        console.log(`Uploaded zip file to S3: ${zipObjectKey}`);
        zipUrls.push(`https://releases.rivet.gg/${zipObjectKey}`);
    }

    return { zipUrls };
}

async function main() {
    await buildCrossPlatform();
    const zipPaths = await generateZipFiles();
    const { zipUrls } = await uploadZipsToS3(zipPaths);
    console.log("Uploaded zip URLs:", zipUrls);
}

if (import.meta.main) {
    main();
}
