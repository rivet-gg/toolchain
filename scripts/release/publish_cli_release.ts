#!/usr/bin/env -S deno run --allow-net --allow-env --allow-read

import { S3Bucket } from "https://deno.land/x/s3@0.5.0/mod.ts";
import { assert } from "https://deno.land/std@0.182.0/testing/asserts.ts";

const BUCKET_NAME = "rivet-releases";
const BUCKET_FOLDER = "cli";

async function publishRelease(releaseName: string) {
  const bucket = new S3Bucket({
    accessKeyID: Deno.env.get("AWS_ACCESS_KEY_ID")!,
    secretKey: Deno.env.get("AWS_SECRET_ACCESS_KEY")!,
    bucket: BUCKET_NAME,
    region: "auto",
    endpointURL: "https://2a94c6a0ced8d35ea63cddc86c2681e7.r2.cloudflarestorage.com/rivet-releases",
  });

  const files = [
    {
      name: "rivet-cli-aarch64-mac",
      path: "target/aarch64-apple-darwin/release/rivet",
    },
    {
      name: "rivet-cli-x86-linux",
      path: "target/x86_64-unknown-linux-gnu/release/rivet",
    },
    {
      name: "rivet-cli-x86-mac",
      path: "target/x86_64-apple-darwin/release/rivet",
    },
    {
      name: "rivet-cli-x86-windows.exe",
      path: "target/x86_64-pc-windows-gnu/release/rivet.exe",
    },
  ];

  for (const { name, path } of files) {
    const objectKey = `${BUCKET_FOLDER}/v${releaseName}/${name}`;

    console.log(`Uploading ${name} to ${BUCKET_NAME}/${objectKey}...`);
    await bucket.putObject(objectKey, await Deno.readFile(path));
    console.log(`Uploaded ${name}`);
  }

  console.log(`Finished publishing release ${releaseName}.`);
}

const release = Deno.env.get("RELEASE_NAME");
if (!release) {
  console.error("Please provide a release name via the RELEASE_NAME environment variable");
  Deno.exit(1);
}

await publishRelease(release);
