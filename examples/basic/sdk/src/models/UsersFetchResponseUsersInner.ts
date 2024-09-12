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
 * @interface UsersFetchResponseUsersInner
 */
export interface UsersFetchResponseUsersInner {
    [key: string]: any | any;
    /**
     * 
     * @type {string}
     * @memberof UsersFetchResponseUsersInner
     */
    id: string;
    /**
     * 
     * @type {string}
     * @memberof UsersFetchResponseUsersInner
     */
    username: string;
    /**
     * 
     * @type {string}
     * @memberof UsersFetchResponseUsersInner
     */
    createdAt: string;
    /**
     * 
     * @type {string}
     * @memberof UsersFetchResponseUsersInner
     */
    updatedAt: string;
}

/**
 * Check if a given object implements the UsersFetchResponseUsersInner interface.
 */
export function instanceOfUsersFetchResponseUsersInner(value: object): value is UsersFetchResponseUsersInner {
    if (!('id' in value) || value['id'] === undefined) return false;
    if (!('username' in value) || value['username'] === undefined) return false;
    if (!('createdAt' in value) || value['createdAt'] === undefined) return false;
    if (!('updatedAt' in value) || value['updatedAt'] === undefined) return false;
    return true;
}

export function UsersFetchResponseUsersInnerFromJSON(json: any): UsersFetchResponseUsersInner {
    return UsersFetchResponseUsersInnerFromJSONTyped(json, false);
}

export function UsersFetchResponseUsersInnerFromJSONTyped(json: any, ignoreDiscriminator: boolean): UsersFetchResponseUsersInner {
    if (json == null) {
        return json;
    }
    return {
        
            ...json,
        'id': json['id'],
        'username': json['username'],
        'createdAt': json['createdAt'],
        'updatedAt': json['updatedAt'],
    };
}

export function UsersFetchResponseUsersInnerToJSON(value?: UsersFetchResponseUsersInner | null): any {
    if (value == null) {
        return value;
    }
    return {
        
            ...value,
        'id': value['id'],
        'username': value['username'],
        'createdAt': value['createdAt'],
        'updatedAt': value['updatedAt'],
    };
}

