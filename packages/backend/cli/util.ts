import { z } from "zod";
import { MigrateMode } from "../toolchain/build/mod.ts";
import { Project } from "../toolchain/project/mod.ts";
import { UserError } from "../toolchain/error/mod.ts";

export const migrateModeSchema = z.enum([
	MigrateMode.Dev,
	MigrateMode.Generate,
	MigrateMode.GenerateAndApply,
	MigrateMode.Apply,
]);

export function resolveModules(project: Project, moduleNames: string[]) {
	if (moduleNames.length > 0) {
		return moduleNames.map((name) => {
			const module = project.modules.get(name);
			if (!module) throw new UserError(`Module not found: ${name}`);
			return module;
		});
	} else {
		return Array.from(project.modules.values());
	}
}
