export async function build() {
	// const bundledFile = projectDataPath(project, BUNDLE_PATH);
	//
	// // See Cloudflare Wrangler implementation:
	// //
	// // https://github.com/cloudflare/workers-sdk/blob/e8997b879605fb2eabc3e241086feb7aa219ef03/packages/wrangler/src/deployment-bundle/bundle.ts#L276
	// const analyzeResult = Deno.env.get("_BACKEND_ESBUILD_META") == "1";
	// const noMinify = Deno.env.get("_BACKEND_ESBUILD_NO_MINIFY") == "1";
	// const result = await esbuild.build({
	// 	entryPoints: [projectDataPath(project, ENTRYPOINT_PATH)],
	// 	outfile: bundledFile,
	// 	format: "esm",
	// 	sourcemap: true,
	// 	plugins: [
	// 		// Bundle Deno dependencies
	// 		...denoPlugins(),
	//
	// 		// HACK: esbuild-deno-loader does not play nice with
	// 		// Windows paths, so we manually resolve any paths that
	// 		// start with a Windows path separator (\) and resolve
	// 		// them to the full path.
	// 		{
	// 			name: "fix-windows-paths",
	// 			setup(build: esbuild.PluginBuild) {
	// 				build.onResolve({ filter: /^\\.*/ }, (args) => {
	// 					const resolvedPath = resolve(args.resolveDir, args.path);
	// 					if (!exists(resolvedPath, { isFile: true })) {
	// 						return {
	// 							errors: [{ text: `File could not be resolved: ${resolvedPath}` }],
	// 						};
	// 					}
	//
	// 					return {
	// 						path: resolve(args.resolveDir, args.path),
	// 					};
	// 				});
	// 			},
	// 		} satisfies esbuild.Plugin,
	//
	// 		// Remove unused Node imports
	// 		nodeModulesPolyfillPlugin({
	// 			globals: {
	// 				Buffer: true,
	// 				process: true,
	// 			},
	// 			modules: {
	// 				// Not used:
	// 				// https://github.com/brianc/node-postgres/blob/50c06f9bc6ff2ca1e8d7b7268b9af54ce49d72c1/packages/pg/lib/crypto/utils.js#L3
	// 				crypto: "empty",
	// 				dns: "empty",
	// 				events: true,
	// 				fs: "empty",
	// 				net: "empty",
	// 				path: "empty",
	// 				string_decoder: true,
	// 				tls: "empty",
	// 				buffer: true,
	// 			},
	// 		}),
	// 	],
	// 	define: {
	// 		// HACK: Disable `process.domain` in order to correctly handle this edge case:
	// 		// https://github.com/brianc/node-postgres/blob/50c06f9bc6ff2ca1e8d7b7268b9af54ce49d72c1/packages/pg/lib/native/query.js#L126
	// 		"process.domain": "undefined",
	// 	},
	// 	external: [
	// 		// Check supported compat by Cloudflare Workers:
	// 		// https://developers.cloudflare.com/workers/runtime-apis/nodejs/
	// 		"node:process",
	// 		"node:stream",
	// 		"node:util",
	//
	// 		// TODO: Why is node:crypto not working? Are any of these external imports working?
	// 		// https://community.cloudflare.com/t/not-being-able-to-import-node-crypto/613973
	// 		// "node:crypto",
	//
	// 		// pg-native is overridden with pg-cloudflare at runtime
	// 		"pg-native",
	//
	// 		// Wasm must be loaded as a separate file manually, cannot be bundled
	// 		"*.wasm",
	// 		"*.wasm?module",
	//
	// 		// This import only exists when running on cloudflare
	// 		"cloudflare:*",
	// 	],
	// 	bundle: true,
	// 	minify: !noMinify,
	//
	// 	logLevel: analyzeResult ? "debug" : "error",
	// 	metafile: analyzeResult,
	// });
	//
	// if (result.metafile) {
	// 	console.log(await esbuild.analyzeMetafile(result.metafile));
	// }
	//
	// if (opts.runtime == Runtime.CloudflareWorkersPlatforms) {
	// 	const bundleStr = await Deno.readTextFile(bundledFile);
	//
	// 	// TODO: Add ability for injecting WASM modules
	// 	// // Find any `query-engine.wasm`
	// 	// let wasmPath;
	// 	// for (const module of project.modules.values()) {
	// 	// 	const moduleWasmPath = resolve(
	// 	// 		genPrismaOutputFolder(project, module),
	// 	// 		"query_engine_bg.wasm",
	// 	// 	);
	// 	//
	// 	// 	if (await exists(moduleWasmPath)) {
	// 	// 		wasmPath = moduleWasmPath;
	// 	// 		break;
	// 	// 	}
	// 	// }
	// 	//
	// 	// // Check if wasm is actually required
	// 	// if (wasmPath) {
	// 	// 	// Make wasm import relative
	// 	// 	bundleStr = bundleStr.replaceAll(
	// 	// 		/file:[\w\\/\.\-]+query_engine_bg\.wasm/g,
	// 	// 		"query-engine.wasm",
	// 	// 	);
	// 	// } else if (/file:[\w\\/\.\-]+query_engine_bg\.wasm/.test(bundleStr)) {
	// 	// 	throw new InternalError("Failed to find required query_engine_bg.wasm", { path: bundledFile });
	// 	// }
	//
	// 	await Deno.writeTextFile(bundledFile, bundleStr);
	//
	// 	// Write manifest of file paths for easier upload from Rivet CLI
	// 	//
	// 	// These modules are relative to the project root in case this was
	// 	// generated from a Docker container.
	// 	const manifest = {
	// 		bundle: relative(project.path, bundledFile),
	// 		wasm: undefined,
	// 		// wasm: wasmPath ? relative(project.path, wasmPath) : undefined,
	// 	};
	//
	// 	await Deno.writeTextFile(
	// 		projectDataPath(project, OUTPUT_MANIFEST_PATH),
	// 		JSON.stringify(manifest),
	// 	);
	// }
}
