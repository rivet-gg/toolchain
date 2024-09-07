import { z } from "zod";
import * as build from "./commands/build.ts";
import * as clean from "./commands/clean.ts";
import * as configShow from "./commands/config/show.ts";
import * as createModule from "./commands/create/module.ts";
import * as createTest from "./commands/create/test.ts";
import * as dev from "./commands/dev.ts";
import * as format from "./commands/format.ts";
import * as init from "./commands/init.ts";
import * as lint from "./commands/lint.ts";
import * as sdkGenerate from "./commands/sdk/generate.ts";
import * as test from "./commands/test.ts";
import * as dbInstanceStart from "./commands/db/instance/start.ts";
import * as dbInstanceStatus from "./commands/db/instance/status.ts";
import * as dbInstanceStop from "./commands/db/instance/stop.ts";
import * as dbMigrateApply from "./commands/db/migrate/apply.ts";
import * as dbMigrateDrop from "./commands/db/migrate/drop.ts";
import * as dbMigrateGenerate from "./commands/db/migrate/generate.ts";
import * as dbMigratePush from "./commands/db/migrate/push.ts";
import * as dbReset from "./commands/db/reset.ts";
import * as dbSh from "./commands/db/sh.ts";
import * as dbUrl from "./commands/db/url.ts";
import { UnreachableError } from "../toolchain/error/mod.ts";

export const commandSchema = z.union([
	z.object({ build: build.optsSchema }),
	z.object({ clean: clean.optsSchema }),
	z.object({ configShow: configShow.optsSchema }),
	z.object({ createModule: createModule.optsSchema }),
	z.object({ createTest: createTest.optsSchema }),
	z.object({ dev: dev.optsSchema }),
	z.object({ format: format.optsSchema }),
	z.object({ init: init.optsSchema }),
	z.object({ lint: lint.optsSchema }),
	z.object({ sdkGenerate: sdkGenerate.optsSchema }),
	z.object({ test: test.optsSchema }),
	z.object({ dbInstanceStart: dbInstanceStart.optsSchema }),
	z.object({ dbInstanceStatus: dbInstanceStatus.optsSchema }),
	z.object({ dbInstanceStop: dbInstanceStop.optsSchema }),
	z.object({ dbMigrateApply: dbMigrateApply.optsSchema }),
	z.object({ dbMigrateDrop: dbMigrateDrop.optsSchema }),
	z.object({ dbMigrateGenerate: dbMigrateGenerate.optsSchema }),
	z.object({ dbMigratePush: dbMigratePush.optsSchema }),
	z.object({ dbReset: dbReset.optsSchema }),
	z.object({ dbSh: dbSh.optsSchema }),
	z.object({ dbUrl: dbUrl.optsSchema }),
]);

type Command = z.infer<typeof commandSchema>;

export async function executeCommand(command: Command) {
	if ("build" in command) {
		await build.execute(command.build);
	} else if ("clean" in command) {
		await clean.execute(command.clean);
	} else if ("configShow" in command) {
		await configShow.execute(command.configShow);
	} else if ("createModule" in command) {
		await createModule.execute(command.createModule);
	} else if ("createTest" in command) {
		await createTest.execute(command.createTest);
	} else if ("dev" in command) {
		await dev.execute(command.dev);
	} else if ("format" in command) {
		await format.execute(command.format);
	} else if ("init" in command) {
		await init.execute(command.init);
	} else if ("lint" in command) {
		await lint.execute(command.lint);
	} else if ("sdkGenerate" in command) {
		await sdkGenerate.execute(command.sdkGenerate);
	} else if ("test" in command) {
		await test.execute(command.test);
	} else if ("dbInstanceStart" in command) {
		await dbInstanceStart.execute(command.dbInstanceStart);
	} else if ("dbInstanceStatus" in command) {
		await dbInstanceStatus.execute(command.dbInstanceStatus);
	} else if ("dbInstanceStop" in command) {
		await dbInstanceStop.execute(command.dbInstanceStop);
	} else if ("dbMigrateApply" in command) {
		await dbMigrateApply.execute(command.dbMigrateApply);
	} else if ("dbMigrateDrop" in command) {
		await dbMigrateDrop.execute(command.dbMigrateDrop);
	} else if ("dbMigrateGenerate" in command) {
		await dbMigrateGenerate.execute(command.dbMigrateGenerate);
	} else if ("dbMigratePush" in command) {
		await dbMigratePush.execute(command.dbMigratePush);
	} else if ("dbReset" in command) {
		await dbReset.execute(command.dbReset);
	} else if ("dbSh" in command) {
		await dbSh.execute(command.dbSh);
	} else if ("dbUrl" in command) {
		await dbUrl.execute(command.dbUrl);
	} else {
		throw new UnreachableError(command);
	}
}
