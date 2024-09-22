import { exists } from "@std/fs";
import { InternalError } from "../error/mod.ts";
import { addShutdownHandler } from "./shutdown_handler.ts";
import { error } from "../term/status.ts";

export interface FsLock {
	file: Deno.FsFile;
	path: string;
	released: boolean;
}

export async function acquireLock(opts: { path: string; onWaiting?: () => void }): Promise<FsLock> {
	const { path, onWaiting } = opts;

	let file: Deno.FsFile | null = null;

	// Check if the lock already exists in order to provide status to user, even
	// if the lock is not valid
	if (await exists(path, { isFile: true })) {
		onWaiting?.();
	}

	// Lock file
	try {
		file = await Deno.open(path, { create: true, write: true });
		await file.lock(true);
	} catch (cause) {
		if (file) file.close();
		throw new InternalError("Failed to acquire file lock", { cause });
	}

	// Build lock
	const lock = { file, path, released: false };

	// TODO: This will cause a memory leak if creating a lot of locks
	// Automatically release
	addShutdownHandler(() => releaseLock(lock));

	return lock;
}

export async function releaseLock(lock: FsLock) {
	if (lock.released) {
		return;
	}

	try {
		await lock.file.unlock();
		lock.file.close();
	} catch (err) {
		error("Error releasing lock", `${err}`);
	}

  try {
		await Deno.remove(lock.path);
  } catch (_err) {
    // Do nothing. This file may have been deleted for many reasons (i.e.
    // cleaning project).
  }

	lock.released = true;
}
