// This file is auto-generated by the Open Game Backend (https://opengb.dev) build system.
//
// Do not edit this file directly.
//
// Generated at 2024-09-12T00:20:07.027Z

/* tslint:disable */
/* eslint-disable */

/**
 * @export
 * @interface TokensFetchRequest
 */
export interface TokensFetchRequest {}

/**
 * Check if a given object implements the TokensFetchRequest interface.
 */
export function instanceOfTokensFetchRequest(
  _value: object,
): _value is TokensFetchRequest {
  return true;
}

export function TokensFetchRequestFromJSON(json: any): TokensFetchRequest {
  return TokensFetchRequestFromJSONTyped(json, false);
}

export function TokensFetchRequestFromJSONTyped(
  json: any,
  ignoreDiscriminator: boolean,
): TokensFetchRequest {
  if (json == null) {
    return json;
  }
  return {};
}

export function TokensFetchRequestToJSON(
  value?: TokensFetchRequest | null,
): any {
  if (value == null) {
    return value;
  }
  return {};
}
