// This file is auto-generated by the Rivet (https://rivet.gg) build system.
//
// Do not edit this file directly.

import * as runtime from "../../runtime";

import type { UsersFetchRequest, UsersFetchResponse } from "../../models/index";
import {
  UsersFetchRequestToJSON,
  UsersFetchResponseFromJSON,
} from "../../models/index";

import type {
  UsersFetchByUsernameRequest,
  UsersFetchByUsernameResponse,
} from "../../models/index";
import {
  UsersFetchByUsernameRequestToJSON,
  UsersFetchByUsernameResponseFromJSON,
} from "../../models/index";

import type {
  UsersCreateRequest,
  UsersCreateResponse,
} from "../../models/index";
import {
  UsersCreateRequestToJSON,
  UsersCreateResponseFromJSON,
} from "../../models/index";

import type {
  UsersAuthenticateTokenRequest,
  UsersAuthenticateTokenResponse,
} from "../../models/index";
import {
  UsersAuthenticateTokenRequestToJSON,
  UsersAuthenticateTokenResponseFromJSON,
} from "../../models/index";

import type {
  UsersCreateTokenRequest,
  UsersCreateTokenResponse,
} from "../../models/index";
import {
  UsersCreateTokenRequestToJSON,
  UsersCreateTokenResponseFromJSON,
} from "../../models/index";

export class Users extends runtime.BaseAPI {
  public async fetch(
    request: UsersFetchRequest,
    initOverrides?: RequestInit | runtime.InitOverrideFunction,
  ): Promise<UsersFetchResponse> {
    const queryParameters: any = {};

    const headerParameters: runtime.HTTPHeaders = {};
    headerParameters["Content-Type"] = "application/json";

    const response = await this.request({
      path: `/modules/users/scripts/fetch/call`,
      method: "POST",
      headers: headerParameters,
      query: queryParameters,
      body: UsersFetchRequestToJSON(request),
    }, initOverrides);

    return UsersFetchResponseFromJSON(await response.json());
  }

  public async fetchByUsername(
    request: UsersFetchByUsernameRequest,
    initOverrides?: RequestInit | runtime.InitOverrideFunction,
  ): Promise<UsersFetchByUsernameResponse> {
    const queryParameters: any = {};

    const headerParameters: runtime.HTTPHeaders = {};
    headerParameters["Content-Type"] = "application/json";

    const response = await this.request({
      path: `/modules/users/scripts/fetch_by_username/call`,
      method: "POST",
      headers: headerParameters,
      query: queryParameters,
      body: UsersFetchByUsernameRequestToJSON(request),
    }, initOverrides);

    return UsersFetchByUsernameResponseFromJSON(await response.json());
  }

  public async create(
    request: UsersCreateRequest,
    initOverrides?: RequestInit | runtime.InitOverrideFunction,
  ): Promise<UsersCreateResponse> {
    const queryParameters: any = {};

    const headerParameters: runtime.HTTPHeaders = {};
    headerParameters["Content-Type"] = "application/json";

    const response = await this.request({
      path: `/modules/users/scripts/create/call`,
      method: "POST",
      headers: headerParameters,
      query: queryParameters,
      body: UsersCreateRequestToJSON(request),
    }, initOverrides);

    return UsersCreateResponseFromJSON(await response.json());
  }

  public async authenticateToken(
    request: UsersAuthenticateTokenRequest,
    initOverrides?: RequestInit | runtime.InitOverrideFunction,
  ): Promise<UsersAuthenticateTokenResponse> {
    const queryParameters: any = {};

    const headerParameters: runtime.HTTPHeaders = {};
    headerParameters["Content-Type"] = "application/json";

    const response = await this.request({
      path: `/modules/users/scripts/authenticate_token/call`,
      method: "POST",
      headers: headerParameters,
      query: queryParameters,
      body: UsersAuthenticateTokenRequestToJSON(request),
    }, initOverrides);

    return UsersAuthenticateTokenResponseFromJSON(await response.json());
  }

  public async createToken(
    request: UsersCreateTokenRequest,
    initOverrides?: RequestInit | runtime.InitOverrideFunction,
  ): Promise<UsersCreateTokenResponse> {
    const queryParameters: any = {};

    const headerParameters: runtime.HTTPHeaders = {};
    headerParameters["Content-Type"] = "application/json";

    const response = await this.request({
      path: `/modules/users/scripts/create_token/call`,
      method: "POST",
      headers: headerParameters,
      query: queryParameters,
      body: UsersCreateTokenRequestToJSON(request),
    }, initOverrides);

    return UsersCreateTokenResponseFromJSON(await response.json());
  }
}