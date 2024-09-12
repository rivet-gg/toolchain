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
 * @interface FriendsSendRequestRequest
 */
export interface FriendsSendRequestRequest {
    /**
     * 
     * @type {string}
     * @memberof FriendsSendRequestRequest
     */
    userToken: string;
    /**
     * 
     * @type {string}
     * @memberof FriendsSendRequestRequest
     */
    targetUserId: string;
}

/**
 * Check if a given object implements the FriendsSendRequestRequest interface.
 */
export function instanceOfFriendsSendRequestRequest(value: object): value is FriendsSendRequestRequest {
    if (!('userToken' in value) || value['userToken'] === undefined) return false;
    if (!('targetUserId' in value) || value['targetUserId'] === undefined) return false;
    return true;
}

export function FriendsSendRequestRequestFromJSON(json: any): FriendsSendRequestRequest {
    return FriendsSendRequestRequestFromJSONTyped(json, false);
}

export function FriendsSendRequestRequestFromJSONTyped(json: any, ignoreDiscriminator: boolean): FriendsSendRequestRequest {
    if (json == null) {
        return json;
    }
    return {
        
        'userToken': json['userToken'],
        'targetUserId': json['targetUserId'],
    };
}

export function FriendsSendRequestRequestToJSON(value?: FriendsSendRequestRequest | null): any {
    if (value == null) {
        return value;
    }
    return {
        
        'userToken': value['userToken'],
        'targetUserId': value['targetUserId'],
    };
}

