// This file is auto-generated by the Rivet (https://rivet.gg) build system.
//
// Do not edit this file directly.

/* tslint:disable */
/* eslint-disable */

/**
 * @export
 * @interface TokensRevokeResponse
 */
export interface TokensRevokeResponse {}

/**
 * Check if a given object implements the TokensRevokeResponse interface.
 */
export function instanceOfTokensRevokeResponse(
  _value: object,
): _value is TokensRevokeResponse {
  return true;
}

export function TokensRevokeResponseFromJSON(json: any): TokensRevokeResponse {
  return TokensRevokeResponseFromJSONTyped(json, false);
}

export function TokensRevokeResponseFromJSONTyped(
  json: any,
  ignoreDiscriminator: boolean,
): TokensRevokeResponse {
  if (json == null) {
    return json;
  }
  return {};
}

export function TokensRevokeResponseToJSON(
  value?: TokensRevokeResponse | null,
): any {
  if (value == null) {
    return value;
  }
  return {};
}
