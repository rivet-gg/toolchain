import { newTrace } from "../../../mod.ts";
import { ScheduleDriver } from "../../driver.ts";
import { ActorRecord, MemoryActorDriver } from "./driver.ts";

export class MemorySchedule implements ScheduleDriver {
	constructor(private readonly driver: MemoryActorDriver, private readonly actorRecord: ActorRecord) {}

	after(duration: number, fn: string, request: unknown): void {
		this._setTimeout(duration, fn, request);
	}

	at(timestamp: number, fn: string, request: unknown): void {
		this._setTimeout(timestamp - Date.now(), fn, request);
	}

	_setTimeout(duration: number, fn: string, request: unknown): void {
		const timeoutId = setTimeout(() => {
			this.actorRecord.timeoutIds.delete(timeoutId);
			this.driver.callActor({
				moduleName: this.actorRecord.moduleName,
				actorName: this.actorRecord.actorName,
				instanceName: this.actorRecord.instanceName,
				fn,
				request,
				trace: newTrace({ actorSchedule: {} }),
			});
		}, duration);
		this.actorRecord.timeoutIds.add(timeoutId);
	}

	async __inspect(): Promise<any> {
		return {};
	}
}
