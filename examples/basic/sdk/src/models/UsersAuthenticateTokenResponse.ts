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
import type { UsersFetchResponseUsersInner } from './UsersFetchResponseUsersInner';
import {
    UsersFetchResponseUsersInnerFromJSON,
    UsersFetchResponseUsersInnerFromJSONTyped,
    UsersFetchResponseUsersInnerToJSON,
} from './UsersFetchResponseUsersInner';

/**
 * 
 * @export
 * @interface UsersAuthenticateTokenResponse
 */
export interface UsersAuthenticateTokenResponse {
    /**
     * 
     * @type {string}
     * @memberof UsersAuthenticateTokenResponse
     */
    userId: string;
    /**
     * 
     * @type {UsersFetchResponseUsersInner}
     * @memberof UsersAuthenticateTokenResponse
     */
    user?: UsersFetchResponseUsersInner;
}

/**
 * Check if a given object implements the UsersAuthenticateTokenResponse interface.
 */
export function instanceOfUsersAuthenticateTokenResponse(value: object): value is UsersAuthenticateTokenResponse {
    if (!('userId' in value) || value['userId'] === undefined) return false;
    return true;
}

export function UsersAuthenticateTokenResponseFromJSON(json: any): UsersAuthenticateTokenResponse {
    return UsersAuthenticateTokenResponseFromJSONTyped(json, false);
}

export function UsersAuthenticateTokenResponseFromJSONTyped(json: any, ignoreDiscriminator: boolean): UsersAuthenticateTokenResponse {
    if (json == null) {
        return json;
    }
    return {
        
        'userId': json['userId'],
        'user': json['user'] == null ? undefined : UsersFetchResponseUsersInnerFromJSON(json['user']),
    };
}

export function UsersAuthenticateTokenResponseToJSON(value?: UsersAuthenticateTokenResponse | null): any {
    if (value == null) {
        return value;
    }
    return {
        
        'userId': value['userId'],
        'user': UsersFetchResponseUsersInnerToJSON(value['user']),
    };
}

