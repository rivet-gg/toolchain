// This file is auto-generated by the Open Game Backend (https://opengb.dev) build system.
//
// Do not edit this file directly.
//
// Generated at 2024-09-12T00:20:07.987Z

/* tslint:disable */
/* eslint-disable */

/**
 * @export
 * @interface TokensValidateResponse
 */
export interface TokensValidateResponse {}

/**
 * Check if a given object implements the TokensValidateResponse interface.
 */
export function instanceOfTokensValidateResponse(
  _value: object,
): _value is TokensValidateResponse {
  return true;
}

export function TokensValidateResponseFromJSON(
  json: any,
): TokensValidateResponse {
  return TokensValidateResponseFromJSONTyped(json, false);
}

export function TokensValidateResponseFromJSONTyped(
  json: any,
  ignoreDiscriminator: boolean,
): TokensValidateResponse {
  if (json == null) {
    return json;
  }
  return {};
}

export function TokensValidateResponseToJSON(
  value?: TokensValidateResponse | null,
): any {
  if (value == null) {
    return value;
  }
  return {};
}
