import { z } from "zod";

export const globalOptsSchema = z.object({}).catchall(z.unknown());

export type GlobalOpts = z.infer<typeof globalOptsSchema>;
