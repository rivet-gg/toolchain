import { z } from "zod";
import { UnreachableError } from "../toolchain/error/mod.ts";

import * as build from "./commands/build.ts";
import * as clean from "./commands/clean.ts";
import * as configShow from "./commands/config/show.ts";
import * as configManifestPath from "./commands/config/manifest_path.ts";
import * as configOutputManifestPath from "./commands/config/output_manifest_path.ts";
import * as createActor from "./commands/create/actor.ts";
import * as createModule from "./commands/create/module.ts";
import * as createScript from "./commands/create/script.ts";
import * as createTest from "./commands/create/test.ts";
import * as dbMigrateApply from "./commands/db/migrate/apply.ts";
import * as dbMigrateDrop from "./commands/db/migrate/drop.ts";
import * as dbMigrateGenerate from "./commands/db/migrate/generate.ts";
import * as dbMigratePush from "./commands/db/migrate/push.ts";
import * as dev from "./commands/dev.ts";
import * as format from "./commands/format.ts";
import * as genOpenApi from "./commands/gen_openapi.ts";
import * as init from "./commands/init.ts";
import * as lint from "./commands/lint.ts";
import * as test from "./commands/test.ts";

export const commandSchema = z.union([
	z.object({ build: build.optsSchema }),
	z.object({ clean: clean.optsSchema }),
	z.object({ configShow: configShow.optsSchema }),
	z.object({ configManifestPath: configManifestPath.optsSchema }),
	z.object({ configOutputManifestPath: configOutputManifestPath.optsSchema }),
	z.object({ createActor: createActor.optsSchema }),
	z.object({ createModule: createModule.optsSchema }),
	z.object({ createScript: createScript.optsSchema }),
	z.object({ createTest: createTest.optsSchema }),
	z.object({ dev: dev.optsSchema }),
	z.object({ format: format.optsSchema }),
	z.object({ genOpenApi: genOpenApi.optsSchema }),
	z.object({ init: init.optsSchema }),
	z.object({ lint: lint.optsSchema }),
	z.object({ test: test.optsSchema }),
	z.object({ dbMigrateApply: dbMigrateApply.optsSchema }),
	z.object({ dbMigrateDrop: dbMigrateDrop.optsSchema }),
	z.object({ dbMigrateGenerate: dbMigrateGenerate.optsSchema }),
	z.object({ dbMigratePush: dbMigratePush.optsSchema }),
]);

type Command = z.infer<typeof commandSchema>;

export async function executeCommand(command: Command) {
	if ("build" in command) {
		await build.execute(command.build);
	} else if ("clean" in command) {
		await clean.execute(command.clean);
	} else if ("configShow" in command) {
		await configShow.execute(command.configShow);
	} else if ("createActor" in command) {
		await createActor.execute(command.createActor);
	} else if ("createModule" in command) {
		await createModule.execute(command.createModule);
	} else if ("createScript" in command) {
		await createScript.execute(command.createScript);
	} else if ("createTest" in command) {
		await createTest.execute(command.createTest);
	} else if ("dev" in command) {
		await dev.execute(command.dev);
	} else if ("format" in command) {
		await format.execute(command.format);
	} else if ("genOpenApi" in command) {
		await genOpenApi.execute(command.genOpenApi);
	} else if ("init" in command) {
		await init.execute(command.init);
	} else if ("configManifestPath" in command) {
		await configManifestPath.execute(command.configManifestPath);
	} else if ("configOutputManifestPath" in command) {
		await configOutputManifestPath.execute(command.configOutputManifestPath);
	} else if ("lint" in command) {
		await lint.execute(command.lint);
	} else if ("test" in command) {
		await test.execute(command.test);
	} else if ("dbMigrateApply" in command) {
		await dbMigrateApply.execute(command.dbMigrateApply);
	} else if ("dbMigrateDrop" in command) {
		await dbMigrateDrop.execute(command.dbMigrateDrop);
	} else if ("dbMigrateGenerate" in command) {
		await dbMigrateGenerate.execute(command.dbMigrateGenerate);
	} else if ("dbMigratePush" in command) {
		await dbMigratePush.execute(command.dbMigratePush);
	} else {
		throw new UnreachableError(command);
	}
}
