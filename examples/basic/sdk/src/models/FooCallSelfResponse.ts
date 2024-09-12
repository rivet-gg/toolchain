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
import type { FooCallSelfResponseResponse } from './FooCallSelfResponseResponse';
import {
    FooCallSelfResponseResponseFromJSON,
    FooCallSelfResponseResponseFromJSONTyped,
    FooCallSelfResponseResponseToJSON,
} from './FooCallSelfResponseResponse';

/**
 * 
 * @export
 * @interface FooCallSelfResponse
 */
export interface FooCallSelfResponse {
    [key: string]: any | any;
    /**
     * 
     * @type {FooCallSelfResponseResponse}
     * @memberof FooCallSelfResponse
     */
    response: FooCallSelfResponseResponse;
}

/**
 * Check if a given object implements the FooCallSelfResponse interface.
 */
export function instanceOfFooCallSelfResponse(value: object): value is FooCallSelfResponse {
    if (!('response' in value) || value['response'] === undefined) return false;
    return true;
}

export function FooCallSelfResponseFromJSON(json: any): FooCallSelfResponse {
    return FooCallSelfResponseFromJSONTyped(json, false);
}

export function FooCallSelfResponseFromJSONTyped(json: any, ignoreDiscriminator: boolean): FooCallSelfResponse {
    if (json == null) {
        return json;
    }
    return {
        
            ...json,
        'response': FooCallSelfResponseResponseFromJSON(json['response']),
    };
}

export function FooCallSelfResponseToJSON(value?: FooCallSelfResponse | null): any {
    if (value == null) {
        return value;
    }
    return {
        
            ...value,
        'response': FooCallSelfResponseResponseToJSON(value['response']),
    };
}

