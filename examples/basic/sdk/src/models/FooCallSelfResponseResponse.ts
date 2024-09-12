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
 * @interface FooCallSelfResponseResponse
 */
export interface FooCallSelfResponseResponse {
    [key: string]: any | any;
    /**
     * 
     * @type {string}
     * @memberof FooCallSelfResponseResponse
     */
    pong: string;
}

/**
 * Check if a given object implements the FooCallSelfResponseResponse interface.
 */
export function instanceOfFooCallSelfResponseResponse(value: object): value is FooCallSelfResponseResponse {
    if (!('pong' in value) || value['pong'] === undefined) return false;
    return true;
}

export function FooCallSelfResponseResponseFromJSON(json: any): FooCallSelfResponseResponse {
    return FooCallSelfResponseResponseFromJSONTyped(json, false);
}

export function FooCallSelfResponseResponseFromJSONTyped(json: any, ignoreDiscriminator: boolean): FooCallSelfResponseResponse {
    if (json == null) {
        return json;
    }
    return {
        
            ...json,
        'pong': json['pong'],
    };
}

export function FooCallSelfResponseResponseToJSON(value?: FooCallSelfResponseResponse | null): any {
    if (value == null) {
        return value;
    }
    return {
        
            ...value,
        'pong': value['pong'],
    };
}

