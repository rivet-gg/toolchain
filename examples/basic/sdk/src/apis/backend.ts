// This file is auto-generated by the Open Game Backend (https://opengb.dev) build system.
//
// Do not edit this file directly.
//
// Generated at 2024-09-12T00:20:09.294Z

import * as runtime from "../runtime";

import { Users } from "./modules/users";

import { RateLimit } from "./modules/rate_limit";

import { Tokens } from "./modules/tokens";

import { Foo } from "./modules/foo";

import { ConfigTest } from "./modules/config_test";

import { Friends } from "./modules/friends";

export class Backend extends runtime.BaseAPI {
  constructor(config: runtime.ConfigurationParameters) {
    super(new runtime.Configuration(config));
  }

  protected _users: Users | undefined;

  public get users(): Users {
    return this._users ??= new Users(this.configuration);
  }

  protected _rateLimit: RateLimit | undefined;

  public get rateLimit(): RateLimit {
    return this._rateLimit ??= new RateLimit(this.configuration);
  }

  protected _tokens: Tokens | undefined;

  public get tokens(): Tokens {
    return this._tokens ??= new Tokens(this.configuration);
  }

  protected _foo: Foo | undefined;

  public get foo(): Foo {
    return this._foo ??= new Foo(this.configuration);
  }

  protected _configTest: ConfigTest | undefined;

  public get configTest(): ConfigTest {
    return this._configTest ??= new ConfigTest(this.configuration);
  }

  protected _friends: Friends | undefined;

  public get friends(): Friends {
    return this._friends ??= new Friends(this.configuration);
  }
}
