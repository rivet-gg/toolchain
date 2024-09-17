import { z } from "zod";
import { globalOptsSchema, initProject } from "../common.ts";
import { generateOpenApi } from "../../toolchain/build/openapi.ts";
import { build, DbDriver, Format, Runtime } from "../../toolchain/build/mod.ts";

export const optsSchema = globalOptsSchema.extend({
  output: z.string()
});

type Opts = z.infer<typeof optsSchema>;

export async function execute(opts: Opts) {
	const project = await initProject(opts);

  await build(project, {
    format: Format.Native,
    runtime: Runtime.Deno,
    dbDriver: DbDriver.NodePostgres,
    // Require schemas to be generated in order to build OpenAPI types
    strictSchemas: true,
    skipDenoCheck: true,
  });

  await generateOpenApi(project, opts.output);
}
