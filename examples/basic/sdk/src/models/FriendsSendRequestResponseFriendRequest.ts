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
 * @interface FriendsSendRequestResponseFriendRequest
 */
export interface FriendsSendRequestResponseFriendRequest {
    [key: string]: any | any;
    /**
     * 
     * @type {string}
     * @memberof FriendsSendRequestResponseFriendRequest
     */
    id: string;
    /**
     * 
     * @type {string}
     * @memberof FriendsSendRequestResponseFriendRequest
     */
    senderUserId: string;
    /**
     * 
     * @type {string}
     * @memberof FriendsSendRequestResponseFriendRequest
     */
    targetUserId: string;
    /**
     * 
     * @type {string}
     * @memberof FriendsSendRequestResponseFriendRequest
     */
    createdAt: string;
    /**
     * 
     * @type {string}
     * @memberof FriendsSendRequestResponseFriendRequest
     */
    declinedAt: string;
    /**
     * 
     * @type {string}
     * @memberof FriendsSendRequestResponseFriendRequest
     */
    acceptedAt: string;
}

/**
 * Check if a given object implements the FriendsSendRequestResponseFriendRequest interface.
 */
export function instanceOfFriendsSendRequestResponseFriendRequest(value: object): value is FriendsSendRequestResponseFriendRequest {
    if (!('id' in value) || value['id'] === undefined) return false;
    if (!('senderUserId' in value) || value['senderUserId'] === undefined) return false;
    if (!('targetUserId' in value) || value['targetUserId'] === undefined) return false;
    if (!('createdAt' in value) || value['createdAt'] === undefined) return false;
    if (!('declinedAt' in value) || value['declinedAt'] === undefined) return false;
    if (!('acceptedAt' in value) || value['acceptedAt'] === undefined) return false;
    return true;
}

export function FriendsSendRequestResponseFriendRequestFromJSON(json: any): FriendsSendRequestResponseFriendRequest {
    return FriendsSendRequestResponseFriendRequestFromJSONTyped(json, false);
}

export function FriendsSendRequestResponseFriendRequestFromJSONTyped(json: any, ignoreDiscriminator: boolean): FriendsSendRequestResponseFriendRequest {
    if (json == null) {
        return json;
    }
    return {
        
            ...json,
        'id': json['id'],
        'senderUserId': json['senderUserId'],
        'targetUserId': json['targetUserId'],
        'createdAt': json['createdAt'],
        'declinedAt': json['declinedAt'],
        'acceptedAt': json['acceptedAt'],
    };
}

export function FriendsSendRequestResponseFriendRequestToJSON(value?: FriendsSendRequestResponseFriendRequest | null): any {
    if (value == null) {
        return value;
    }
    return {
        
            ...value,
        'id': value['id'],
        'senderUserId': value['senderUserId'],
        'targetUserId': value['targetUserId'],
        'createdAt': value['createdAt'],
        'declinedAt': value['declinedAt'],
        'acceptedAt': value['acceptedAt'],
    };
}

