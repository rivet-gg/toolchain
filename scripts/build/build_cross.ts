#!/usr/bin/env -S deno run -A

import { resolve } from "jsr:@std/path";
import { ensureDir } from "jsr:@std/fs";

interface Platform {
	name: string;
	target: string;
	files: string[];
}

const REPO_DIR = resolve(import.meta.dirname!, "..", "..");
const DOCKER_IMAGE = "rust-cross-compiler";
const DOCKERFILE = `
FROM rust:1.81
RUN apt-get update && apt-get install -y \\
    gcc-mingw-w64-x86-64 \\
    gcc-x86-64-linux-gnu \\
    libc6-dev-amd64-cross \\
    clang \\
    libssl-dev \\
    wget \\
    xz-utils \\
    cmake \\
    patch \\
    libxml2-dev \\
    llvm-dev \\
    uuid-dev \\
    libssl-dev \\
    curl \\
    unzip \\
    && rm -rf /var/lib/apt/lists/*

# Install Node.js LTS
RUN curl -fsSL https://deb.nodesource.com/setup_lts.x | bash -
RUN apt-get install -y nodejs

# Install Yarn
RUN corepack enable
RUN corepack prepare yarn@stable --activate

# Install Deno
RUN curl -fsSL https://deno.land/x/install/install.sh | sh
ENV PATH="/root/.deno/bin:$PATH"

# Install osxcross
RUN git config --global --add safe.directory '*'
RUN git clone https://github.com/tpoechtrager/osxcross /root/osxcross
WORKDIR /root/osxcross
RUN wget -nc https://github.com/phracker/MacOSX-SDKs/releases/download/11.3/MacOSX11.3.sdk.tar.xz
RUN mv MacOSX11.3.sdk.tar.xz tarballs/
RUN UNATTENDED=yes OSX_VERSION_MIN=10.7 ./build.sh
ENV PATH="/root/osxcross/target/bin:$PATH"

# Install targets
RUN rustup target add x86_64-unknown-linux-gnu \\
    x86_64-pc-windows-gnu \\
    x86_64-apple-darwin \\
    aarch64-apple-darwin

WORKDIR /app

ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-linux-gnu-gcc
ENV CARGO_TARGET_X86_64_APPLE_DARWIN_LINKER=x86_64-apple-darwin20.4-clang
ENV CARGO_TARGET_AARCH64_APPLE_DARWIN_LINKER=aarch64-apple-darwin20.4-clang
ENV CC_x86_64_apple_darwin=x86_64-apple-darwin20.4-clang
ENV CXX_x86_64_apple_darwin=x86_64-apple-darwin20.4-clang++
ENV CC_aarch64_apple_darwin=aarch64-apple-darwin20.4-clang
ENV CXX_aarch64_apple_darwin=aarch64-apple-darwin20.4-clang++

RUN mkdir -p /root/.cargo && \\
    echo '\\
[target.x86_64-unknown-linux-gnu]\\n\\
linker = "x86_64-linux-gnu-gcc"\\n\\
\\n\\
[target.x86_64-pc-windows-gnu]\\n\\
linker = "x86_64-w64-mingw32-gcc"\\n\\
\\n\\
[target.x86_64-apple-darwin]\\n\\
linker = "x86_64-apple-darwin20.4-clang"\\n\\
ar = "x86_64-apple-darwin20.4-ar"\\n\\
\\n\\
[target.aarch64-apple-darwin]\\n\\
linker = "aarch64-apple-darwin20.4-clang"\\n\\
ar = "aarch64-apple-darwin20.4-ar"\\n\\
' > /root/.cargo/config.toml
`;

async function buildDockerImage() {
	console.log("Building Docker image...");
	const command = new Deno.Command("docker", {
		args: ["build", "-t", DOCKER_IMAGE, "-"],
		stdin: "piped",
	});
	const process = command.spawn();
	const writer = process.stdin.getWriter();
	await writer.write(new TextEncoder().encode(DOCKERFILE));
	await writer.close();
	const { code } = await process.output();
	if (code !== 0) {
		throw new Error("Docker build failed");
	}
}

async function buildAndCopyCrossPlatform(
	outDir: string,
	packages: string[] = [],
) {
	console.log("Building and copying cross-platform...");
	await Deno.remove(outDir, { recursive: true }).catch(() => {});

	const platforms: Platform[] = [
		{
		    name: "linux_x86_64",
		    target: "x86_64-unknown-linux-gnu",
		    files: [],
		},
		{
			name: "windows_x86_64",
			target: "x86_64-pc-windows-gnu",
			files: [],
		},
		{
		    name: "macos_x86_64",
		    target: "x86_64-apple-darwin",
		    files: [],
		},
		{
			name: "macos_arm64",
			target: "aarch64-apple-darwin",
			files: [],
		},
	];

	// Determine which files to include based on the packages
	const includeAll = packages.length === 0 || packages.includes("all");
	for (const platform of platforms) {
		if (includeAll || packages.includes("rivet-cli")) {
			platform.files.push(
				platform.name.includes("windows") ? "rivet.exe" : "rivet",
			);
		}
		if (includeAll || packages.includes("rivet-toolchain-ffi")) {
			let ffiLibrary: string;
			if (platform.name.includes("windows")) {
				ffiLibrary = "rivet_toolchain_ffi.dll";
			} else if (platform.name.includes("linux")) {
				ffiLibrary = "librivet_toolchain_ffi.so";
			} else if (platform.name.includes("macos")) {
				ffiLibrary = "librivet_toolchain_ffi.dylib";
			} else {
				throw new Error(`Unsupported platform: ${platform.name}`);
			}
			platform.files.push(ffiLibrary);
		}
	}

	for (const platform of platforms) {
		console.log(`Building for ${platform.name}...`);
		const dockerArgs = [
			"run",
			"--rm",
			"-v",
			`${REPO_DIR}:/app`,
			"-e",
			`OVERRIDE_TARGET=${platform.target}`,
		];

		// Add GITHUB_TOKEN if it exists
		const githubToken = Deno.env.get("GITHUB_TOKEN");
		if (githubToken) {
			dockerArgs.push("-e", `GITHUB_TOKEN=${githubToken}`);
		}

		dockerArgs.push(
			DOCKER_IMAGE,
			"/bin/sh",
			"-c",
			`cargo build -vv --manifest-path Cargo.toml --target ${platform.target} --release && chown -R ${Deno.uid()}:${Deno.gid()} /app/target`,
		);

		const command = new Deno.Command("docker", {
			args: dockerArgs,
			stdin: "inherit",
			stdout: "inherit",
			stderr: "inherit",
		});

		const { code } = await command.output();

		if (code !== 0) {
			throw new Error(`Build failed for ${platform.name}`);
		}

		for (const file of platform.files) {
			const srcPath = resolve(
				REPO_DIR,
				"target",
				platform.target,
				"release",
				file,
			);
			const destPath = resolve(REPO_DIR, outDir, platform.name, file);
			await ensureDir(resolve(REPO_DIR, outDir, platform.name));
			await Deno.copyFile(srcPath, destPath);
			console.log(`Copied ${srcPath} to ${destPath}`);
		}

		// Delete target if needed
		if (Deno.env.get("CROSS_DELETE_TARGET") == "1") {
			const targetPath = resolve(REPO_DIR, "target", platform.target);
			await Deno.remove(targetPath, { recursive: true });
			console.log(`Deleted ${targetPath}`);
		}
	}
}

export async function buildCross(outDir: string, packages: string[] = []) {
	await buildDockerImage();
	await buildAndCopyCrossPlatform(outDir, packages);
}

if (import.meta.main) {
	const dir = await Deno.makeTempDir();
  console.log(dir);
	await buildCross(dir);
}
