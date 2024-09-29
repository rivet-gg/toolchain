import { Configuration } from "./configuration.ts";
import { RivetRequestError } from "./error.ts";
import { Logger } from "./logger.ts";

export interface ClientConfiguration {
	endpoint?: string;
	gameVersion?: string;
}

/** Low-level API used to build HTTP requests to the backend. */
export class Client {
	constructor(private configuration: Configuration) {}

	/** Builds the headers for a request */
	private _buildHeaders(): HeadersInit {
		return {
			"Accept": "application/json",
			"Content-Type": "application/json",
		};
	}

	/** Builds the complete URL to the backend */
	private _buildUrl(path: string): string {
		return `${this.configuration.endpoint}${path}`;
	}

	/** Creates a request */
	async buildRequest(
		requestName: string | null,
		method: string,
		path: string,
		body: any,
	): Promise<any> {
		const url = this._buildUrl(path);
		const headers = this._buildHeaders();
		const bodyJson = JSON.stringify(body);
		const startedAt = performance.now();

		let response: Response | null = null;
		let responseBody: any = null;

		try {
			response = await fetch(url, {
				method: method,
				headers: headers,
				body: bodyJson,
			});

			const elapsed = performance.now() - startedAt;

			// Try to parse the response body as JSON
			try {
				responseBody = await response.json();
			} catch (jsonError) {
				responseBody = null;
			}

			// Log the request and response
			this._logRequest(requestName, url, response, responseBody, elapsed);

			if (response.ok) {
				return responseBody;
			} else {
				// Handle HTTP errors
				const errorCode = responseBody?.code || "unknown_error";
				const errorMessage = responseBody?.message || response.statusText ||
					"Unknown error";
				const errorMeta = responseBody?.meta;

				throw new RivetRequestError(
					errorCode,
					errorMessage,
					response.status,
					errorMeta,
				);
			}
		} catch (error) {
			const elapsed = performance.now() - startedAt;

			// Log the error
			this._logRequest(requestName, url, response, responseBody, elapsed, true);

			// Re-throw the error
			throw error;
		}
	}

	private _logRequest(
		requestName: string | null,
		url: string,
		response: Response | null,
		responseBody: any,
		elapsed: number,
		isErrorOverride = false,
	) {
		let logStr: string;
		let isError = false;

		if (requestName != null) {
			logStr = `request=${requestName}`;
		} else {
			logStr = `request=${url}`;
		}

		if (response && response.ok && responseBody != null) {
			logStr += " result=ok";
		} else if (
			response &&
			(response.status === 400 || response.status === 500) &&
			responseBody != null &&
			"message" in responseBody
		) {
			if ("code" in responseBody) {
				logStr += ` result=${responseBody.code}`;
			} else {
				logStr += " result=unknown_error";
			}
			if ("module" in responseBody) {
				logStr += ` module=${responseBody.module}`;
			}
			logStr += ` message="${responseBody.message}"`;
			if ("meta" in responseBody) {
				logStr += ` meta=${JSON.stringify(responseBody.meta)}`;
			}
		} else {
			isError = true;
			const statusText = response ? response.statusText : "unknown";
			const status = response ? response.status : "unknown";
			logStr += ` result=${statusText} http_status=${status} response_code=${status}`;
		}

		logStr += ` elapsed=${elapsed}ms`;

		if (isError || isErrorOverride) {
			Logger.error(logStr);
		} else {
			Logger.log(logStr);
		}
	}
}
