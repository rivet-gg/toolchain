/* tslint:disable */
/* eslint-disable */
/**
 * Open Game Backend
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
 * @interface FooActorRequest
 */
export interface FooActorRequest {
    /**
     * 
     * @type {string}
     * @memberof FooActorRequest
     */
    id?: string;
}

/**
 * Check if a given object implements the FooActorRequest interface.
 */
export function instanceOfFooActorRequest(value: object): value is FooActorRequest {
    return true;
}

export function FooActorRequestFromJSON(json: any): FooActorRequest {
    return FooActorRequestFromJSONTyped(json, false);
}

export function FooActorRequestFromJSONTyped(json: any, ignoreDiscriminator: boolean): FooActorRequest {
    if (json == null) {
        return json;
    }
    return {
        
        'id': json['id'] == null ? undefined : json['id'],
    };
}

export function FooActorRequestToJSON(value?: FooActorRequest | null): any {
    if (value == null) {
        return value;
    }
    return {
        
        'id': value['id'],
    };
}

