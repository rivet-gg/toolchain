/* tslint:disable */
/* eslint-disable */
/**
 * Rivet SDK
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { mapValues } from '../runtime';
/**
 * 
 * @export
 * @interface FriendsRemoveFriendResponse
 */
export interface FriendsRemoveFriendResponse {
    [key: string]: any;
}

/**
 * Check if a given object implements the FriendsRemoveFriendResponse interface.
 */
export function instanceOfFriendsRemoveFriendResponse(value: object): value is FriendsRemoveFriendResponse {
    return true;
}

export function FriendsRemoveFriendResponseFromJSON(json: any): FriendsRemoveFriendResponse {
    return FriendsRemoveFriendResponseFromJSONTyped(json, false);
}

export function FriendsRemoveFriendResponseFromJSONTyped(json: any, ignoreDiscriminator: boolean): FriendsRemoveFriendResponse {
    return json;
}

export function FriendsRemoveFriendResponseToJSON(value?: FriendsRemoveFriendResponse | null): any {
    return value;
}

