// This file is auto-generated by the Open Game Backend (https://opengb.dev) build system.
//
// Do not edit this file directly.
//
// Generated at 2024-09-12T00:20:08.113Z

/* tslint:disable */
/* eslint-disable */

/**
 * @export
 * @interface TokensExtendRequest
 */
export interface TokensExtendRequest {}

/**
 * Check if a given object implements the TokensExtendRequest interface.
 */
export function instanceOfTokensExtendRequest(
  _value: object,
): _value is TokensExtendRequest {
  return true;
}

export function TokensExtendRequestFromJSON(json: any): TokensExtendRequest {
  return TokensExtendRequestFromJSONTyped(json, false);
}

export function TokensExtendRequestFromJSONTyped(
  json: any,
  ignoreDiscriminator: boolean,
): TokensExtendRequest {
  if (json == null) {
    return json;
  }
  return {};
}

export function TokensExtendRequestToJSON(
  value?: TokensExtendRequest | null,
): any {
  if (value == null) {
    return value;
  }
  return {};
}
