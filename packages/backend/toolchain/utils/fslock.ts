export interface FsLock {
  path: string;
  released: boolean;
  pid: number;
}

export async function acquireLock(opts: { path: string; onWaiting?: () => void }): Promise<FsLock> {
  const { path, onWaiting } = opts;
  const currentPid = Deno.pid;

  let calledWaiting = false;
  while (true) {
    try {
      // Check if lock exists & if pid of lock still exists
      const existingPidStr = await Deno.readTextFile(path).catch(() => null);
      const existingPid = existingPidStr ? parseInt(existingPidStr) : null;
      if (existingPid !== null) {
        if (await processExists(existingPid)) {
          // Call waiting callback
          if (!calledWaiting) {
            onWaiting?.();
            calledWaiting = true;
          }

          // Wait before retrying
          await new Promise((resolve) => setTimeout(resolve, 1000));
          continue;
        }
      }

      // Write this pid to the lock path
      await Deno.writeTextFile(path, currentPid.toString());
      await Deno.chmod(path, 0o600);

      return { path, released: false, pid: currentPid };
    } catch (error) {
      // If the file already exists, retry the loop
      if (!(error instanceof Deno.errors.AlreadyExists)) {
        throw error;
      }
    }
  }
}

export async function releaseLock(lock: FsLock) {
  // Ensure not released
  if (lock.released) {
    return;
  }

  try {
    // Check lockfile equals this pid
    const filePid = await Deno.readTextFile(lock.path).then(Number);
    if (filePid !== lock.pid) {
      throw new Error("Lock file does not match this lock");
    }

    // Delete lockfile
    await Deno.remove(lock.path);

    // Mark as released
    lock.released = true;
  } catch (error) {
    if (!(error instanceof Deno.errors.NotFound)) {
      throw error;
    }
    // If the file is not found, consider it already released
    lock.released = true;
  }
}

async function processExists(pid: number): Promise<boolean> {
  if (!(pid > 0)) return false;
  try {
    await Deno.kill(pid, "SIGINFO");

    // No error, process still exists
    return true;
  } catch (err) {
    // NotFound: not exists, PermissionDenied: exists not ours
    return (err as Deno.errors.NotFound).name !== "NotFound";
  }
}
