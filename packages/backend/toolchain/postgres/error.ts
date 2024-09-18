import { InternalError, InternalErrorOpts } from "../error/mod.ts";

export class ArchiveError extends InternalError {
	constructor(opts?: InternalErrorOpts) {
		super("An error occurred with the archive.", opts);
		this.name = "ArchiveError";
	}
}

export interface CommandErrorOpts extends InternalErrorOpts {
	stdout: string;
	stderr: string;
}

export class CommandError extends InternalError {
	public readonly stdout: string;
	public readonly stderr: string;

	constructor(message: string, opts: CommandErrorOpts) {
		super(message, opts);
		this.name = "CommandError";
		this.stdout = opts.stdout;
		this.stderr = opts.stderr;
	}
}

export class DatabaseError extends InternalError {
	constructor(opts?: InternalErrorOpts) {
		super(`A general database error occurred: ${opts?.originalError ?? "?"}`, opts);
		this.name = "DatabaseError";
	}
}

export class DatabaseInitializationError extends InternalError {
	constructor(opts?: InternalErrorOpts) {
		super("An error occurred during database initialization.", opts);
		this.name = "DatabaseInitializationError";
	}
}

export class DatabaseStartError extends InternalError {
	constructor(opts?: InternalErrorOpts) {
		super("An error occurred while starting the database.", opts);
		this.name = "DatabaseStartError";
	}
}

export class DatabaseStopError extends InternalError {
	constructor(opts?: InternalErrorOpts) {
		super("An error occurred while stopping the database.", opts);
		this.name = "DatabaseStopError";
	}
}

export interface InvalidUrlOpts extends InternalErrorOpts {
	url: string;
}

export class InvalidUrlError extends InternalError {
	public readonly url: string;

	constructor(message: string, opts: InvalidUrlOpts) {
		super(message, opts);
		this.name = "InvalidUrlError";
		this.url = opts.url;
	}
}

export class IoError extends InternalError {
	constructor(opts?: InternalErrorOpts) {
		super("An I/O error occurred.", opts);
		this.name = "IoError";
	}
}

export class ParseError extends InternalError {
	constructor(opts?: InternalErrorOpts) {
		super("A parsing error occurred.", opts);
		this.name = "ParseError";
	}
}

export interface ExecutableNotFoundOpts extends InternalErrorOpts {
	path: string;
}

export class ExecutableNotFoundError extends InternalError {
	public readonly path: string;

	constructor(message: string, opts: ExecutableNotFoundOpts) {
		super(message, opts);
		this.name = "ExecutableNotFoundError";
		this.path = opts.path;
	}
}
