// This file is auto-generated by the Rivet (https://rivet.gg) build system.
//
// Do not edit this file directly.

/* tslint:disable */
/* eslint-disable */

/**
 * @export
 * @interface RateLimitThrottleRequest
 */
export interface RateLimitThrottleRequest {}

/**
 * Check if a given object implements the RateLimitThrottleRequest interface.
 */
export function instanceOfRateLimitThrottleRequest(
  _value: object,
): _value is RateLimitThrottleRequest {
  return true;
}

export function RateLimitThrottleRequestFromJSON(
  json: any,
): RateLimitThrottleRequest {
  return RateLimitThrottleRequestFromJSONTyped(json, false);
}

export function RateLimitThrottleRequestFromJSONTyped(
  json: any,
  ignoreDiscriminator: boolean,
): RateLimitThrottleRequest {
  if (json == null) {
    return json;
  }
  return {};
}

export function RateLimitThrottleRequestToJSON(
  value?: RateLimitThrottleRequest | null,
): any {
  if (value == null) {
    return value;
  }
  return {};
}