import { z } from "zod";
import { globalOptsSchema, initProject } from "../common.ts";
import { cleanProject } from "../../toolchain/project/project.ts";
import { runTask } from "../task.ts";

runTask({
  inputSchema: globalOptsSchema,
  async run(input) {
    const project = await initProject(input);
    await cleanProject(project);

  }
})
