import { createNanoEvents } from "nanoevents";
import { Project } from "../project/mod.ts";

export type ValidationError = {
	name: "ValidationError";
	info: Record<string, string>;
};
type Error = { name: "CombinedError"; errors: Error[] } | ValidationError;

export type State =
	& { project: Project }
	& ({
		value: "idle";
	} | {
		value: "building";
	} | {
		value: "success";
	} | {
		value: "failure";
		error: Error | unknown;
	});

interface Events {
	changed: (state: State) => void;
}

export class InternalState {
	private emitter = createNanoEvents<Events>();

	private state: State = { value: "idle" };

	on<E extends keyof Events>(event: E, callback: Events[E]) {
		return this.emitter.on(event, callback);
	}
	once<E extends keyof Events>(event: E, callback: Events[E]) {
		const unbind = this.emitter.on(event, (...args) => {
			unbind();
			callback(...args);
		});
		return unbind;
	}

	get() {
		return this.state;
	}

	set(state: State) {
		this.state = state;
		this.emitter.emit("changed", state);
	}
}
