# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.0.0-rc.6](https://github.com/rivet-gg/toolchain/compare/v2.0.0-rc.5...v2.0.0-rc.6) (2024-10-03)


### Features

* add `rivet backend generate-sdk` command ([#548](https://github.com/rivet-gg/toolchain/issues/548)) ([b10140e](https://github.com/rivet-gg/toolchain/commit/b10140e63eb1b5390bcc0e27427d9df181ddcfd3))
* add editor ([#390](https://github.com/rivet-gg/toolchain/issues/390)) ([101d476](https://github.com/rivet-gg/toolchain/commit/101d4767c70e7e83d5a4996b90d3d22e124f2f66))
* add process manager for game server & opengb dev server ([#349](https://github.com/rivet-gg/toolchain/issues/349)) ([e1d2369](https://github.com/rivet-gg/toolchain/commit/e1d23699887e2d9800050a3e6d2bea389b5172e7))
* **backend:** add better godot module request logging ([#514](https://github.com/rivet-gg/toolchain/issues/514)) ([9d83741](https://github.com/rivet-gg/toolchain/commit/9d8374145ee98c7c9b74800d1a885eaed6741079))
* **backend:** add openapi gen command ([#401](https://github.com/rivet-gg/toolchain/issues/401)) ([08d106c](https://github.com/rivet-gg/toolchain/commit/08d106c695e0945125eb409cd873dbdb4caf39bf))
* **backend:** add project lock to prevent concurrent actions ([#454](https://github.com/rivet-gg/toolchain/issues/454)) ([056d232](https://github.com/rivet-gg/toolchain/commit/056d23281cdb4e4ee7e89dc91b61c9ba29f34172))
* **backend:** allow specifying registry with "github" shorthand & add default local registry ([#389](https://github.com/rivet-gg/toolchain/issues/389)) ([6daed07](https://github.com/rivet-gg/toolchain/commit/6daed07b6d32be0d4deeb0504654b8b66dd3d63f))
* **backend:** generate typescript sdk ([#520](https://github.com/rivet-gg/toolchain/issues/520)) ([daae06d](https://github.com/rivet-gg/toolchain/commit/daae06d668056647f9b3aad4f78fdd8c961d3920))
* **backend:** report alarm errors in actors ([#527](https://github.com/rivet-gg/toolchain/issues/527)) ([e072ac6](https://github.com/rivet-gg/toolchain/commit/e072ac60f1e87cd8fe1210b29a82ffcf8fe099b3))
* **cli:** add `rivet config data-path` command to inspect internal data ([#541](https://github.com/rivet-gg/toolchain/issues/541)) ([5a80398](https://github.com/rivet-gg/toolchain/commit/5a80398c82eaaad2e990f1fbb79a4aec004f1c04))
* scroll to currenlty selected module ([#510](https://github.com/rivet-gg/toolchain/issues/510)) ([c4b5646](https://github.com/rivet-gg/toolchain/commit/c4b5646b6f3488d62a09e405572e3de922c71304))
* **toolchain:** add get-current-version and get-endpoint commands ([#549](https://github.com/rivet-gg/toolchain/issues/549)) ([294b466](https://github.com/rivet-gg/toolchain/commit/294b4669f5d6fb9f404a6109cf6abe02a9421f5d))
* **toolchain:** dispatch backend config update events ([#397](https://github.com/rivet-gg/toolchain/issues/397)) ([4b5eee8](https://github.com/rivet-gg/toolchain/commit/4b5eee8e00db9bfb203340850773a70be9245b44))
* **toolchain:** fetch current build in bootstrap data ([#519](https://github.com/rivet-gg/toolchain/issues/519)) ([2bf993a](https://github.com/rivet-gg/toolchain/commit/2bf993a120b5e1c1d5213f784bbdfd04a48db685))
* update favicon ([#485](https://github.com/rivet-gg/toolchain/issues/485)) ([38dcf9d](https://github.com/rivet-gg/toolchain/commit/38dcf9d606c8db0c42b740a989ac7fa7eec8d682))


### Bug Fixes

* **actor:** clear timeouts on memory actor on destroy ([#407](https://github.com/rivet-gg/toolchain/issues/407)) ([e047ffa](https://github.com/rivet-gg/toolchain/commit/e047ffab221e5eb09b877dee6ac9293188ed33ff))
* add html5 example ([#538](https://github.com/rivet-gg/toolchain/issues/538)) ([04d293b](https://github.com/rivet-gg/toolchain/commit/04d293b1ef44c629e6b648f6c08afea4b76fac06))
* **backend-embed:** force executable permissions on vendored packages ([#482](https://github.com/rivet-gg/toolchain/issues/482)) ([c6fa1ec](https://github.com/rivet-gg/toolchain/commit/c6fa1ecfb406f2de2599c35100365cbcef9b1039))
* **backend:** add compat for calling yarn on windows in build_artifacts.ts ([#414](https://github.com/rivet-gg/toolchain/issues/414)) ([cf3342d](https://github.com/rivet-gg/toolchain/commit/cf3342d8007b531a48e4a21f3a25254a852081d9))
* **backend:** add deno.jsonc to both generated output & local project ([#452](https://github.com/rivet-gg/toolchain/issues/452)) ([3d567b3](https://github.com/rivet-gg/toolchain/commit/3d567b3129bb5feb828284f299758c88faf51cfa))
* **backend:** apply not getting ran ([#366](https://github.com/rivet-gg/toolchain/issues/366)) ([214df69](https://github.com/rivet-gg/toolchain/commit/214df69f2853b1813c9827b8be95878861439457))
* **backend:** correct windows paths for esbuild nodejs polyfills ([#436](https://github.com/rivet-gg/toolchain/issues/436)) ([30831a2](https://github.com/rivet-gg/toolchain/commit/30831a2b2bac87f3f9d6ce95783098d5d605a2fc))
* **backend:** correctly auto-generate local registry ([#394](https://github.com/rivet-gg/toolchain/issues/394)) ([3d271ae](https://github.com/rivet-gg/toolchain/commit/3d271ae5c59a3ee6e48f7718b9a0df2bc0269253))
* **backend:** correctly handle relative path from drizzle db config -&gt; db migrations ([#423](https://github.com/rivet-gg/toolchain/issues/423)) ([d02038d](https://github.com/rivet-gg/toolchain/commit/d02038dd5d3122d1b3f1b934f855132de852c6a8))
* **backend:** failing ProjectOpts.path type check with cli ([#455](https://github.com/rivet-gg/toolchain/issues/455)) ([03e2dca](https://github.com/rivet-gg/toolchain/commit/03e2dcaace99e59a780c9ab0558c48020d7a4d80))
* **backend:** fix `Could not find package ID for importer` ([#464](https://github.com/rivet-gg/toolchain/issues/464)) ([a98076d](https://github.com/rivet-gg/toolchain/commit/a98076d7fa263ccb8e5cbc2d8c9fdc3caab6a5ca))
* **backend:** fix apply migration file globs not working on windows ([#532](https://github.com/rivet-gg/toolchain/issues/532)) ([18bfd83](https://github.com/rivet-gg/toolchain/commit/18bfd8391e3ead6bea5f5819e883050a31407758))
* **backend:** fix conflicting dev & deploy by running them in different contexts ([#477](https://github.com/rivet-gg/toolchain/issues/477)) ([4ebdc90](https://github.com/rivet-gg/toolchain/commit/4ebdc9054f38dffc2f8bf79b158df4750aaba223))
* **backend:** fix generated relative migrations path ([#437](https://github.com/rivet-gg/toolchain/issues/437)) ([6924340](https://github.com/rivet-gg/toolchain/commit/69243403e5161b9be59e64055b9bcc09381d6be2))
* **backend:** fix incorrect status/stop commands ([#420](https://github.com/rivet-gg/toolchain/issues/420)) ([2ddd474](https://github.com/rivet-gg/toolchain/commit/2ddd474eb57c5c759fb97067ada38da29911be8e))
* **backend:** fix module.gen.ts import ordering to prevent "cannot access xxxx before initialization" error ([#373](https://github.com/rivet-gg/toolchain/issues/373)) ([df04fa3](https://github.com/rivet-gg/toolchain/commit/df04fa33baa3bea7f88cac00bb91206660a82876))
* **backend:** fix reading stderr output in build script ([#515](https://github.com/rivet-gg/toolchain/issues/515)) ([0acc2a1](https://github.com/rivet-gg/toolchain/commit/0acc2a125f5ba51c5e79c6fedd96a44a150fd0e3))
* **backend:** force posix paths for artifacts ([#418](https://github.com/rivet-gg/toolchain/issues/418)) ([3a75bd3](https://github.com/rivet-gg/toolchain/commit/3a75bd3eb5d8a340918a9ac9f39cb7bf9d6ffc45))
* **backend:** generate correct actor driver path ([#362](https://github.com/rivet-gg/toolchain/issues/362)) ([2050694](https://github.com/rivet-gg/toolchain/commit/205069495347748e0e0b2ae810a44aa1f711da9e))
* **backend:** handle sigint and sigbreak correctly to run shutdown ([#456](https://github.com/rivet-gg/toolchain/issues/456)) ([1c02121](https://github.com/rivet-gg/toolchain/commit/1c02121bbd9acea523f83e7487fd9d4a40957588))
* **backend:** register missing create commands ([#372](https://github.com/rivet-gg/toolchain/issues/372)) ([8c59e76](https://github.com/rivet-gg/toolchain/commit/8c59e767d4a731d581b4d6fb24809a52931c6c51))
* **backend:** remove use of locks on configManifestPath, configOutputManifestPath, and configShow ([#466](https://github.com/rivet-gg/toolchain/issues/466)) ([85a57c3](https://github.com/rivet-gg/toolchain/commit/85a57c332f847fc02f6b5dffaa6877c9ca6a96b0))
* **backend:** type using drizzle-orm@0.24 instead of 0.33 ([#376](https://github.com/rivet-gg/toolchain/issues/376)) ([c26d53c](https://github.com/rivet-gg/toolchain/commit/c26d53cf6e1365862fe8ac4924b11477dffaf611))
* **backend:** use deep hash of backend for cache purging ([#388](https://github.com/rivet-gg/toolchain/issues/388)) ([21ff88f](https://github.com/rivet-gg/toolchain/commit/21ff88fdfb4567b9c45171ff3595e755b7b5c9b8))
* change docs link ([#509](https://github.com/rivet-gg/toolchain/issues/509)) ([2c35c7d](https://github.com/rivet-gg/toolchain/commit/2c35c7dba6204c463c45fceca6adf77f9424658c))
* **cli:** module list is now correct ([#363](https://github.com/rivet-gg/toolchain/issues/363)) ([30cebad](https://github.com/rivet-gg/toolchain/commit/30cebada45a34c5796821cd4e7c050c09fff9bce))
* **deno-embed:** build script ([#424](https://github.com/rivet-gg/toolchain/issues/424)) ([c2b347d](https://github.com/rivet-gg/toolchain/commit/c2b347da83667585e969021eb88e5712836cafa1))
* **deno-embed:** fix aarch64 target name ([#471](https://github.com/rivet-gg/toolchain/issues/471)) ([65eee2f](https://github.com/rivet-gg/toolchain/commit/65eee2f42bebf8bad720af881621c1d105da6942))
* **deno-embed:** fix deno target on windows ([#415](https://github.com/rivet-gg/toolchain/issues/415)) ([e0890c7](https://github.com/rivet-gg/toolchain/commit/e0890c7c73b23caae9f7b9e198ab5df581d0d1f8))
* **deno-embed:** windows support ([#378](https://github.com/rivet-gg/toolchain/issues/378)) ([99f0ce8](https://github.com/rivet-gg/toolchain/commit/99f0ce809d119cfc297318597ab12e168c55ffb3))
* force lockfile to generate in data dir instead of project dir ([#545](https://github.com/rivet-gg/toolchain/issues/545)) ([da314bc](https://github.com/rivet-gg/toolchain/commit/da314bc67f91f2538bcd6133cecc0dccaef0df76))
* re-enable cross platform builds ([#551](https://github.com/rivet-gg/toolchain/issues/551)) ([2a6316f](https://github.com/rivet-gg/toolchain/commit/2a6316fabcefa285aabe1bec056b05b4c49b60fa))
* replace rename -&gt; copy in build script ([#542](https://github.com/rivet-gg/toolchain/issues/542)) ([b7c9689](https://github.com/rivet-gg/toolchain/commit/b7c96894934285ca7a0775c7fca02a78ac554793))
* **toolchain:** add windows dep ([#379](https://github.com/rivet-gg/toolchain/issues/379)) ([a3da2e0](https://github.com/rivet-gg/toolchain/commit/a3da2e029177b5e0648d901f2df6d2d42f326f41))
* **toolchain:** compilation error on linux ([#525](https://github.com/rivet-gg/toolchain/issues/525)) ([6673b93](https://github.com/rivet-gg/toolchain/commit/6673b939d43cacd6422553998cad57e66ecec9a1))
* **toolchain:** explicitly handle utf8 error ([#446](https://github.com/rivet-gg/toolchain/issues/446)) ([c2c6b92](https://github.com/rivet-gg/toolchain/commit/c2c6b9273039ec090384de618d65147ec8a1d5a8))
* **toolchain:** fix killing old pid & abort log handles for process manager ([#523](https://github.com/rivet-gg/toolchain/issues/523)) ([89d0469](https://github.com/rivet-gg/toolchain/commit/89d0469e587bcadcb92deef41372daf054e471c3))
* **toolchain:** fix odd compilation error for godot ([#346](https://github.com/rivet-gg/toolchain/issues/346)) ([88fb160](https://github.com/rivet-gg/toolchain/commit/88fb160e36da1f32873cca3272cdd154b181d7c2))
* **toolchain:** fix Postgres::start error on windows ([#531](https://github.com/rivet-gg/toolchain/issues/531)) ([056b4d3](https://github.com/rivet-gg/toolchain/commit/056b4d3b451162f7952bae5aba10b7c049c923f7))
* **toolchain:** fix task manager compilation for windows ([#530](https://github.com/rivet-gg/toolchain/issues/530)) ([e0d1325](https://github.com/rivet-gg/toolchain/commit/e0d1325aac6eebcc2017a1ff85758e14868a58a2))
* **toolchain:** fix zombie processes & term window popups on windows ([#473](https://github.com/rivet-gg/toolchain/issues/473)) ([4b63f78](https://github.com/rivet-gg/toolchain/commit/4b63f78bdae5bc7da896f08de2a5c028d65b64f4))
* **toolchain:** force kill existing pid for process manager not stopped cleanly ([#506](https://github.com/rivet-gg/toolchain/issues/506)) ([24ff721](https://github.com/rivet-gg/toolchain/commit/24ff7216a63a355912a45669174b4f13421f9e44))
* **toolchain:** force kill full process tree in process manager ([#507](https://github.com/rivet-gg/toolchain/issues/507)) ([b685ff7](https://github.com/rivet-gg/toolchain/commit/b685ff705d78d670ad048922f81b8a5e5baaeebc))
* **toolchain:** handle progress manager signals correctly on windows ([#430](https://github.com/rivet-gg/toolchain/issues/430)) ([f6fc52d](https://github.com/rivet-gg/toolchain/commit/f6fc52d43acb35f1e7162285173fa034bb158d2e))
* **toolchain:** handle unix zombie processes correctly in process manager ([#425](https://github.com/rivet-gg/toolchain/issues/425)) ([4b5df8c](https://github.com/rivet-gg/toolchain/commit/4b5df8c67f2d06fbf72d2e41fd3e85250ee4d4f2))
* **toolchain:** include backend dir relative to manifest dir ([#356](https://github.com/rivet-gg/toolchain/issues/356)) ([efa1964](https://github.com/rivet-gg/toolchain/commit/efa196428ff6c9e97b6218adfc212e018061e1cc))
* **toolchain:** only inject DATABASE_URL to backend if not already provided ([#481](https://github.com/rivet-gg/toolchain/issues/481)) ([6a6d790](https://github.com/rivet-gg/toolchain/commit/6a6d790fa2376eec020a2a2668ac10f9e59cbbb1))
* **toolchain:** prevent command popups on windows ([#422](https://github.com/rivet-gg/toolchain/issues/422)) ([c0850f5](https://github.com/rivet-gg/toolchain/commit/c0850f5342a18a3ca0132736a2a793aafcddcb46))
* **toolchain:** read output & project manifests without depending on script ([#478](https://github.com/rivet-gg/toolchain/issues/478)) ([f1c14b2](https://github.com/rivet-gg/toolchain/commit/f1c14b24c6c23005a08df504a453a7280d008f21))
* **toolchain:** remove unix import on windows targets ([#484](https://github.com/rivet-gg/toolchain/issues/484)) ([8123c0d](https://github.com/rivet-gg/toolchain/commit/8123c0d8b1b869b4411e77b39e89722d7860b816))
* **toolchain:** remove use of libproc on macos in favor of pkill ([#536](https://github.com/rivet-gg/toolchain/issues/536)) ([7582b7e](https://github.com/rivet-gg/toolchain/commit/7582b7e820585fceda8455e1a60b2ea1fb348129))
* **toolchain:** suppress window creation when running cmd on windows ([#348](https://github.com/rivet-gg/toolchain/issues/348)) ([29ba978](https://github.com/rivet-gg/toolchain/commit/29ba97838822b441dd2dcd5a9c6ce9ef10fa2013))
* **toolchain:** tail logs using cancel-safe line iter ([#448](https://github.com/rivet-gg/toolchain/issues/448)) ([7c5a5e7](https://github.com/rivet-gg/toolchain/commit/7c5a5e709fdd5a80255ea0a0311b457708d94fb9))
* **toolchain:** update path to output manifest ([#438](https://github.com/rivet-gg/toolchain/issues/438)) ([5e84923](https://github.com/rivet-gg/toolchain/commit/5e849234f67343722793a3855acafe09103c4e78))


### Documentation

* add backend internal docs ([#400](https://github.com/rivet-gg/toolchain/issues/400)) ([a501469](https://github.com/rivet-gg/toolchain/commit/a501469a8893fc6226dcd4556ce1a905034629fe))


### Code Refactoring

* remove dependency on FONTAWESOME_PACKAGE_TOKEN ([#447](https://github.com/rivet-gg/toolchain/issues/447)) ([da3e6c7](https://github.com/rivet-gg/toolchain/commit/da3e6c7b2dceedf1b097e0bdb4f487da79099fd8))


### Continuous Integration

* add back release please ([#557](https://github.com/rivet-gg/toolchain/issues/557)) ([1c2d3b6](https://github.com/rivet-gg/toolchain/commit/1c2d3b67fa1f190692a12bb3188fbbf5dacdca07))


### Chores

* 🤫 ([#504](https://github.com/rivet-gg/toolchain/issues/504)) ([d086ad2](https://github.com/rivet-gg/toolchain/commit/d086ad2751c6be07462a2c68414d1224c321fa32))
* add back hidden task command ([#371](https://github.com/rivet-gg/toolchain/issues/371)) ([cff8007](https://github.com/rivet-gg/toolchain/commit/cff8007cac8fbdf8812994ca08483d25fc30507e))
* add build_cross & release script ([#543](https://github.com/rivet-gg/toolchain/issues/543)) ([26606fe](https://github.com/rivet-gg/toolchain/commit/26606fe16095a25c5640e3ed49c543c7e71e0410))
* add docs to cli ([#354](https://github.com/rivet-gg/toolchain/issues/354)) ([41cbc0a](https://github.com/rivet-gg/toolchain/commit/41cbc0a1becbf7bec9d0272860953cf4dbfb4e49))
* add more info to readme ([#555](https://github.com/rivet-gg/toolchain/issues/555)) ([36d4541](https://github.com/rivet-gg/toolchain/commit/36d454135c04190d0b7f84042cd210950bf4c66e))
* add new install instructions ([#554](https://github.com/rivet-gg/toolchain/issues/554)) ([b06f621](https://github.com/rivet-gg/toolchain/commit/b06f62190a18186be34f5f4dedfaadb16042235d))
* allow non-existent local registry dirs ([#395](https://github.com/rivet-gg/toolchain/issues/395)) ([c8d6899](https://github.com/rivet-gg/toolchain/commit/c8d6899b9699ba5971733040e2f0acb874f5c080))
* auto-build backend artifacts in build.rs ([#359](https://github.com/rivet-gg/toolchain/issues/359)) ([3143ad9](https://github.com/rivet-gg/toolchain/commit/3143ad9608cb62107d74e1dd35e5fe862fb1f4b0))
* auto-generate sdk on build ([#382](https://github.com/rivet-gg/toolchain/issues/382)) ([fc21e59](https://github.com/rivet-gg/toolchain/commit/fc21e595117e87918609ea63776cd57aefbb60b1))
* auto-pick editor port ([#391](https://github.com/rivet-gg/toolchain/issues/391)) ([265a2ad](https://github.com/rivet-gg/toolchain/commit/265a2ad875d68c8cfd43c7259a942e8d514b2d48))
* **backend-embed:** output deno logs ([#393](https://github.com/rivet-gg/toolchain/issues/393)) ([fecda97](https://github.com/rivet-gg/toolchain/commit/fecda97bf971c514c452c8bbe3eb8ccdcbad3e7d))
* **backend:** add global opts to backend command wrappers ([#406](https://github.com/rivet-gg/toolchain/issues/406)) ([8f523bf](https://github.com/rivet-gg/toolchain/commit/8f523bf7b33b1db8f558d08f2ee2ff65ae1b18ce))
* **backend:** add message on migrate command ([#364](https://github.com/rivet-gg/toolchain/issues/364)) ([f1c34be](https://github.com/rivet-gg/toolchain/commit/f1c34beccc38ec14e747de957903a71a3df6e671))
* **backend:** add postgres connection timeout ([#474](https://github.com/rivet-gg/toolchain/issues/474)) ([5b100aa](https://github.com/rivet-gg/toolchain/commit/5b100aa5e085d2c2798dc29fa68d91a161a7304a))
* **backend:** add releasing project ([#458](https://github.com/rivet-gg/toolchain/issues/458)) ([736d94c](https://github.com/rivet-gg/toolchain/commit/736d94c7f28c5679c16e1092e73a00f031d6b47e))
* **backend:** allow sdk overriding ([#387](https://github.com/rivet-gg/toolchain/issues/387)) ([e7c8dc1](https://github.com/rivet-gg/toolchain/commit/e7c8dc157058b44e140187cf01f9eb680387cfc0))
* **backend:** auto-generate deno.jsonc to modules directory ([#462](https://github.com/rivet-gg/toolchain/issues/462)) ([f2a0cd0](https://github.com/rivet-gg/toolchain/commit/f2a0cd025635d0646e9a6d2b5ad85a741b737886))
* **backend:** bump modules ([#439](https://github.com/rivet-gg/toolchain/issues/439)) ([7594640](https://github.com/rivet-gg/toolchain/commit/75946402fb1043ac0cbd1db93ad6562182dc6b53))
* **backend:** cleanly handle errors in cli ([#365](https://github.com/rivet-gg/toolchain/issues/365)) ([3a895e8](https://github.com/rivet-gg/toolchain/commit/3a895e84e2e3fb9e60f371384b21cba314a74792))
* **backend:** connect all abort controllers to shutdown ([#459](https://github.com/rivet-gg/toolchain/issues/459)) ([d4b774c](https://github.com/rivet-gg/toolchain/commit/d4b774c5f7c4eec702041d1369973b5427d16ae7))
* **backend:** cut embed size by ~75% ([#544](https://github.com/rivet-gg/toolchain/issues/544)) ([f90ffb6](https://github.com/rivet-gg/toolchain/commit/f90ffb6884e9fed43b9cfdcf155811193dff38ea))
* **backend:** default to strict schemas in tests ([#408](https://github.com/rivet-gg/toolchain/issues/408)) ([c740d61](https://github.com/rivet-gg/toolchain/commit/c740d61414129f48b136ee47008a75ea2f6f954e))
* **backend:** fix unable to resolve cloudflare:workers dependency ([#451](https://github.com/rivet-gg/toolchain/issues/451)) ([ecadb0c](https://github.com/rivet-gg/toolchain/commit/ecadb0c949105f72d95af869214ebea0bed02816))
* **backend:** fmt ([#534](https://github.com/rivet-gg/toolchain/issues/534)) ([5b5fb26](https://github.com/rivet-gg/toolchain/commit/5b5fb26c5240b6756ba5caead71fba9be45ef28b))
* **backend:** remove .gitignore now that cache path is not src dir ([#457](https://github.com/rivet-gg/toolchain/issues/457)) ([b9b3d92](https://github.com/rivet-gg/toolchain/commit/b9b3d92efc9e2751dd9e2d4b76430183c9a413e8))
* **backend:** remove archived json in favor of reading from fs ([#512](https://github.com/rivet-gg/toolchain/issues/512)) ([c6dcb83](https://github.com/rivet-gg/toolchain/commit/c6dcb83d5fcf3634c7357d0f807adacc5a70eb2f))
* **backend:** simplify cli task running ([#476](https://github.com/rivet-gg/toolchain/issues/476)) ([c1b27d5](https://github.com/rivet-gg/toolchain/commit/c1b27d502d62bd3b7ebedb87af1227cabc894802))
* **backend:** skip logging progress for steps that take very little time ([#461](https://github.com/rivet-gg/toolchain/issues/461)) ([14c3858](https://github.com/rivet-gg/toolchain/commit/14c3858baeda9f69865e857c03de83a87986086f))
* **backend:** template enum values in to command help ([#435](https://github.com/rivet-gg/toolchain/issues/435)) ([35a3d41](https://github.com/rivet-gg/toolchain/commit/35a3d41f103327eb7c465fbd17bcb22ef36e0b1e))
* **backend:** update default modules ([#522](https://github.com/rivet-gg/toolchain/issues/522)) ([96cb495](https://github.com/rivet-gg/toolchain/commit/96cb4950af1418af69a6fd146f3598f39cd0f847))
* **backend:** update modules ([#409](https://github.com/rivet-gg/toolchain/issues/409)) ([5c0981e](https://github.com/rivet-gg/toolchain/commit/5c0981e87d59518fb46886d856d7fcb4aabc52c1))
* **backend:** vendor dependencies ([#450](https://github.com/rivet-gg/toolchain/issues/450)) ([4c5fbd5](https://github.com/rivet-gg/toolchain/commit/4c5fbd5b3eb49f2da05ab27d31403f3ae151bb35))
* bind clap commands to backend commands ([#353](https://github.com/rivet-gg/toolchain/issues/353)) ([5d3281a](https://github.com/rivet-gg/toolchain/commit/5d3281a0c8fa1f918715b9b0345f3e081d8849b3))
* clean up build artifacts script ([#358](https://github.com/rivet-gg/toolchain/issues/358)) ([8cedb29](https://github.com/rivet-gg/toolchain/commit/8cedb2936af6bf25e1cf77d60a734a81acbfadfc))
* clean up html5 demo ([#550](https://github.com/rivet-gg/toolchain/issues/550)) ([7dee61a](https://github.com/rivet-gg/toolchain/commit/7dee61a0facfd2e809340c969caac0c892eeba51))
* **deno-embed:** embed deno in toolchain ([#374](https://github.com/rivet-gg/toolchain/issues/374)) ([2b4e24b](https://github.com/rivet-gg/toolchain/commit/2b4e24b085b4d6f7c782eefc21b5518591056119))
* **deno-embed:** support aarch toolchain ([#469](https://github.com/rivet-gg/toolchain/issues/469)) ([1f16bc6](https://github.com/rivet-gg/toolchain/commit/1f16bc6a3a3215df76c3fd6e894aaa58ee02d1ed))
* **editor:** replace opengb logo with rivet logo ([#410](https://github.com/rivet-gg/toolchain/issues/410)) ([7a87f0c](https://github.com/rivet-gg/toolchain/commit/7a87f0c9ad34fc6e567e75ff092ef42081cc619d))
* embed backend source ([#351](https://github.com/rivet-gg/toolchain/issues/351)) ([ca8fd05](https://github.com/rivet-gg/toolchain/commit/ca8fd05c73a07e286ed8f2788f23bfe9a7957991))
* **ffi:** update to latest task format ([#381](https://github.com/rivet-gg/toolchain/issues/381)) ([b24b82e](https://github.com/rivet-gg/toolchain/commit/b24b82e40e94cd706562c8b16b31f54369bbfc24))
* fix relative migrations path on macOS with realDir ([#440](https://github.com/rivet-gg/toolchain/issues/440)) ([91ef488](https://github.com/rivet-gg/toolchain/commit/91ef48887e1eb9e649cc590966940203eff8a20a))
* fix test command ([#367](https://github.com/rivet-gg/toolchain/issues/367)) ([b15e4b3](https://github.com/rivet-gg/toolchain/commit/b15e4b3eb64c8aa9f881eea087a97e1a0c783aae))
* force lf line endings from git ([#417](https://github.com/rivet-gg/toolchain/issues/417)) ([a8e0079](https://github.com/rivet-gg/toolchain/commit/a8e00794f5f4a7f57fde989b31dd73fdbd5837f6))
* merge backend in to toolchain ([#350](https://github.com/rivet-gg/toolchain/issues/350)) ([c398a2f](https://github.com/rivet-gg/toolchain/commit/c398a2f0d5aa5fe5bb9c742695c02effa0c1ad84))
* migrate from global-error -&gt; anyhow ([#355](https://github.com/rivet-gg/toolchain/issues/355)) ([1a972be](https://github.com/rivet-gg/toolchain/commit/1a972bec7a23a7e5ed91ee3f0473a2ee9999622b))
* move args to postitional for create commands ([#370](https://github.com/rivet-gg/toolchain/issues/370)) ([560260f](https://github.com/rivet-gg/toolchain/commit/560260ff0c067d564208174adad9124ceca0979b))
* move cache to global dir ([#384](https://github.com/rivet-gg/toolchain/issues/384)) ([6267235](https://github.com/rivet-gg/toolchain/commit/62672352df377b5656ecd330f7db35a5d79cc47d))
* move output events to custom event handler fn ([#343](https://github.com/rivet-gg/toolchain/issues/343)) ([3c5acb0](https://github.com/rivet-gg/toolchain/commit/3c5acb06f202f36c10dcf836a34d607939bfb6cb))
* move task logic to cli ([#345](https://github.com/rivet-gg/toolchain/issues/345)) ([c014583](https://github.com/rivet-gg/toolchain/commit/c0145838d7e7ec9c0896f476441a1e6fe244cd9f))
* **postgers:** add connectable status check ([#421](https://github.com/rivet-gg/toolchain/issues/421)) ([426677c](https://github.com/rivet-gg/toolchain/commit/426677c97e9b34f0870154e2ab8914799e954675))
* **postgres:** cleanly handle unconnectable database ([#419](https://github.com/rivet-gg/toolchain/issues/419)) ([801c416](https://github.com/rivet-gg/toolchain/commit/801c4168e451e68364de6aef93015d6b88da4234))
* prefix all env vars with `RIVET_*` ([#392](https://github.com/rivet-gg/toolchain/issues/392)) ([5cf04b3](https://github.com/rivet-gg/toolchain/commit/5cf04b31f6218c86e4924788719f559066d36908))
* preserve generated files in sdk output ([#547](https://github.com/rivet-gg/toolchain/issues/547)) ([f65a267](https://github.com/rivet-gg/toolchain/commit/f65a267758351ff24eb71d0c0aa29f8d96acaab1))
* **process-runner:** fix warnings ([#434](https://github.com/rivet-gg/toolchain/issues/434)) ([e297d34](https://github.com/rivet-gg/toolchain/commit/e297d349909c91a7da30c2540d54d84aca7acbab))
* **process-supervisor-embed:** bump cargo nightly version ([#416](https://github.com/rivet-gg/toolchain/issues/416)) ([5928592](https://github.com/rivet-gg/toolchain/commit/59285920999d814facd0cc1aca084991163b24b5))
* refactor opengb cli commands to take json input ([#352](https://github.com/rivet-gg/toolchain/issues/352)) ([774136f](https://github.com/rivet-gg/toolchain/commit/774136f46fb190356e57366c0f65563311bbd742))
* release 2.0.0-rc.6 ([b1aabde](https://github.com/rivet-gg/toolchain/commit/b1aabdebdd722d1b189979bc156b741384c1e5a7))
* remove required flags for repeated module args ([#368](https://github.com/rivet-gg/toolchain/issues/368)) ([751b957](https://github.com/rivet-gg/toolchain/commit/751b957f8a6786ec1f51f37e48ace61abdfb4a4d))
* remove sdk gen task ([#386](https://github.com/rivet-gg/toolchain/issues/386)) ([ac0386e](https://github.com/rivet-gg/toolchain/commit/ac0386ed43b24cac7e888306b44c15fe84c0515e))
* remove stty check since dev is now intended to be able to run without a tty ([#396](https://github.com/rivet-gg/toolchain/issues/396)) ([1d265b6](https://github.com/rivet-gg/toolchain/commit/1d265b6406339eea6171521c184eff6d482318f0))
* remove uses of "opengb" ([#383](https://github.com/rivet-gg/toolchain/issues/383)) ([dae11fd](https://github.com/rivet-gg/toolchain/commit/dae11fd04435ce54781a68cc031d8a13aac3288d))
* rename backend meta.json -&gt; project_manifest.json ([#399](https://github.com/rivet-gg/toolchain/issues/399)) ([43a0e93](https://github.com/rivet-gg/toolchain/commit/43a0e9355f67322d396a6d738675b4d99b5de135))
* rename backend_source_path -&gt; source_path ([#360](https://github.com/rivet-gg/toolchain/issues/360)) ([ef9724a](https://github.com/rivet-gg/toolchain/commit/ef9724a3fe752b4a41bc76b498927dc872b7b32a))
* rename backend.json -&gt; rivet.json ([#385](https://github.com/rivet-gg/toolchain/issues/385)) ([a89f7fa](https://github.com/rivet-gg/toolchain/commit/a89f7fa1c60320114c36c8745019ea1e628ecd7b))
* rename manifest.json -&gt; output_manifest.json ([#398](https://github.com/rivet-gg/toolchain/issues/398)) ([5735aa0](https://github.com/rivet-gg/toolchain/commit/5735aa0c9f1a22555e3514f694a549b89e01ec70))
* simplify backend_source_path setting ([#357](https://github.com/rivet-gg/toolchain/issues/357)) ([b523dad](https://github.com/rivet-gg/toolchain/commit/b523daddf422c6a9b5b80f4b55c9e6238c244b86))
* switch from openssl -&gt; rustls ([#347](https://github.com/rivet-gg/toolchain/issues/347)) ([d082651](https://github.com/rivet-gg/toolchain/commit/d0826510232a088d330188a325ce29ad0b7d79ad))
* **toolchain-ffi:** prefix ffi with rivet_ for windows naming conflicts ([#413](https://github.com/rivet-gg/toolchain/issues/413)) ([da30e19](https://github.com/rivet-gg/toolchain/commit/da30e196b491919eb57e37c6c5b94590032f357d))
* **toolchain:** add ability to hook to existing process manager tasks & clean up backend port choosing ([#375](https://github.com/rivet-gg/toolchain/issues/375)) ([0cccf30](https://github.com/rivet-gg/toolchain/commit/0cccf3016d351d0d220061a6be4cc6f8e9f342a1))
* **toolchain:** add mutex lock on meta file to prevent race conditions when writing ([#404](https://github.com/rivet-gg/toolchain/issues/404)) ([c259044](https://github.com/rivet-gg/toolchain/commit/c259044eb700183963a56ab8ecb7e45273963c02))
* **toolchain:** add project-specific meta.json files ([#405](https://github.com/rivet-gg/toolchain/issues/405)) ([c00eb72](https://github.com/rivet-gg/toolchain/commit/c00eb72cdf2f592fbeba2922d6facd25b2b8fed4))
* **toolchain:** expliciltly handle runner errors in process manager ([#427](https://github.com/rivet-gg/toolchain/issues/427)) ([89797ac](https://github.com/rivet-gg/toolchain/commit/89797ac0d13b402f037ef28a9658042511523d44))
* **toolchain:** expose sdks in event ([#453](https://github.com/rivet-gg/toolchain/issues/453)) ([5a5f09d](https://github.com/rivet-gg/toolchain/commit/5a5f09d47a282ac9e45b11eca9ed7d73dffb4e64))
* **toolchain:** fix build pipeline in ci ([#442](https://github.com/rivet-gg/toolchain/issues/442)) ([5144a5d](https://github.com/rivet-gg/toolchain/commit/5144a5d9eff6dda4112ae9874016329def969b84))
* **toolchain:** fix no backend output on windows ([#433](https://github.com/rivet-gg/toolchain/issues/433)) ([7599ba8](https://github.com/rivet-gg/toolchain/commit/7599ba8d8786706bf7f0bafe47ba883659638bf4))
* **toolchain:** fix warnings ([#479](https://github.com/rivet-gg/toolchain/issues/479)) ([ffe05de](https://github.com/rivet-gg/toolchain/commit/ffe05deb5bf76ca60b5cfd327b83843a776d80bb))
* **toolchain:** impl killing process tree on unix ([#431](https://github.com/rivet-gg/toolchain/issues/431)) ([29477a5](https://github.com/rivet-gg/toolchain/commit/29477a52ae73b4edf42cfef506b09110bc910ed1))
* **toolchain:** most postgres logic from backend to toolchain ([#475](https://github.com/rivet-gg/toolchain/issues/475)) ([fef96e2](https://github.com/rivet-gg/toolchain/commit/fef96e2b3ed1bc6ff4f1e08360ed46e518a3777d))
* **toolchain:** prevent running postgres as root user to prevent broken state ([#539](https://github.com/rivet-gg/toolchain/issues/539)) ([43515ff](https://github.com/rivet-gg/toolchain/commit/43515ff511d8ff937cd4ec2f2e2631e2ac1de6b1))
* **toolchain:** remove extra logging ([#468](https://github.com/rivet-gg/toolchain/issues/468)) ([c57befe](https://github.com/rivet-gg/toolchain/commit/c57befecca6b9b528dca4e97ae03bc4e33534dba))
* **toolchain:** remove requirement to be signed in for toolchain tasks ([#503](https://github.com/rivet-gg/toolchain/issues/503)) ([ffff590](https://github.com/rivet-gg/toolchain/commit/ffff5900aec8534cbcb9be2bc5a3ce9177497cf6))
* **toolchain:** remove unused get_hub_link task ([#465](https://github.com/rivet-gg/toolchain/issues/465)) ([0c40acc](https://github.com/rivet-gg/toolchain/commit/0c40acc31d0f2d91f1ed2b76eef967512f9379b2))
* **toolchain:** rename process supervisor -&gt; process-runner ([#426](https://github.com/rivet-gg/toolchain/issues/426)) ([d2336b7](https://github.com/rivet-gg/toolchain/commit/d2336b75a8075510fd972780af3e5c8c9144d1f7))
* **toolchain:** reorg project_manifest ([#516](https://github.com/rivet-gg/toolchain/issues/516)) ([acb109d](https://github.com/rivet-gg/toolchain/commit/acb109d2a0bf12b3251611d02a86693b31ee7374))
* **toolchain:** split up os-specific process manager code ([#505](https://github.com/rivet-gg/toolchain/issues/505)) ([b2c0388](https://github.com/rivet-gg/toolchain/commit/b2c0388b59333cb71241e6fb865f5bfaf75bf46b))
* **toolchain:** update default dockerfile path to game_server.Dockerfile ([#463](https://github.com/rivet-gg/toolchain/issues/463)) ([27a4dfd](https://github.com/rivet-gg/toolchain/commit/27a4dfd7455fccf79f07a7e06143dd1a7a3c49f6))
* **toolchain:** update project path name to include file name for easier reference ([#540](https://github.com/rivet-gg/toolchain/issues/540)) ([03af1d9](https://github.com/rivet-gg/toolchain/commit/03af1d96f6283025be614907a6822aafa070e31e))
* **toolchain:** use merkle tree for backend hash ([#513](https://github.com/rivet-gg/toolchain/issues/513)) ([8ae5e86](https://github.com/rivet-gg/toolchain/commit/8ae5e867d2088e60aa93dbed15dc51b112a0325c))
* update godot sdk to rivet modules ([#377](https://github.com/rivet-gg/toolchain/issues/377)) ([2c0fe9e](https://github.com/rivet-gg/toolchain/commit/2c0fe9ec103b5ed3db5ebe52d5bc14e8757f26e2))
* update html5 tutorial to use packages ([#546](https://github.com/rivet-gg/toolchain/issues/546)) ([7d2b711](https://github.com/rivet-gg/toolchain/commit/7d2b7119b743e0f5ae9628e03ad72f12b110a449))
* update modules ([#537](https://github.com/rivet-gg/toolchain/issues/537)) ([2db8a30](https://github.com/rivet-gg/toolchain/commit/2db8a3008db6fed71a6f06cf0ee0d8918e8313f5))
* update readme ([#369](https://github.com/rivet-gg/toolchain/issues/369)) ([9dc2da0](https://github.com/rivet-gg/toolchain/commit/9dc2da046d249f351e50efbfb0ffb1984b556589))
* update release please ([44a8ea1](https://github.com/rivet-gg/toolchain/commit/44a8ea1c3960d9e0081fffdd6f1556dd8728466a))
* update unity codegen to native deno ([#403](https://github.com/rivet-gg/toolchain/issues/403)) ([dc31592](https://github.com/rivet-gg/toolchain/commit/dc3159244d28df20e42af2e0d028ade2412e693c))
* upgrade backend tasks to use new command interface ([#361](https://github.com/rivet-gg/toolchain/issues/361)) ([4615f7f](https://github.com/rivet-gg/toolchain/commit/4615f7fdb8e77ceb57ffd12fd91755f7bd375372))
* use dynamic deno executable path ([#402](https://github.com/rivet-gg/toolchain/issues/402)) ([f1dc38d](https://github.com/rivet-gg/toolchain/commit/f1dc38d84188ed25ac21892c50a15c2ac06b9bab))
* vendor deno dependencies ([#449](https://github.com/rivet-gg/toolchain/issues/449)) ([5f259ba](https://github.com/rivet-gg/toolchain/commit/5f259badaf2d627944908cd325e959398041035b))

## [2.0.0-rc.5](https://github.com/rivet-gg/cli/compare/v2.0.0-rc.4...v2.0.0-rc.5) (2024-08-31)


### Chores

* add portable deno installation ([#341](https://github.com/rivet-gg/cli/issues/341)) ([2d526f4](https://github.com/rivet-gg/cli/commit/2d526f4a9f687e73d4695a3442edcd2584cc4aae))
* release 2.0.0-rc.5 ([613daa1](https://github.com/rivet-gg/cli/commit/613daa153c7a6784de7b6255123ec7a6716a4281))
* remove duplicate build script ([#342](https://github.com/rivet-gg/cli/issues/342)) ([da947e2](https://github.com/rivet-gg/cli/commit/da947e2d1fe72888b2602218ba54f858d059a5a8))
* set msrv for crates ([#338](https://github.com/rivet-gg/cli/issues/338)) ([48f4b82](https://github.com/rivet-gg/cli/commit/48f4b82a54002170bfaa256ff38a881f4e5b0281))
* update migrate command on deploy ([#344](https://github.com/rivet-gg/cli/issues/344)) ([c2d90e2](https://github.com/rivet-gg/cli/commit/c2d90e2435e5db1ded2dae450e8740e05b460a2c))
* update opengb build flag ([#340](https://github.com/rivet-gg/cli/issues/340)) ([75f6105](https://github.com/rivet-gg/cli/commit/75f610568144ff10a20c5659d7068d4695e9c80e))

## [2.0.0-rc.4](https://github.com/rivet-gg/cli/compare/v2.0.0-rc.3...v2.0.0-rc.4) (2024-08-22)


### Features

* **deploy:** deploy with tags ([#304](https://github.com/rivet-gg/cli/issues/304)) ([ffac4a9](https://github.com/rivet-gg/cli/commit/ffac4a9e85e6bbf38727f9b0be98615da37035e2))
* reduce cli binary size ([#332](https://github.com/rivet-gg/cli/issues/332)) ([a9ea424](https://github.com/rivet-gg/cli/commit/a9ea42459d84b442aeac6c2f631cb116be7829a0))


### Bug Fixes

* add tty only for backend ([#326](https://github.com/rivet-gg/cli/issues/326)) ([b2f0ba0](https://github.com/rivet-gg/cli/commit/b2f0ba0ed80662cf1e6f3ab247369577eeca941a))
* **backend:** add postgres mounts for bundled postgres ([#330](https://github.com/rivet-gg/cli/issues/330)) ([85cc352](https://github.com/rivet-gg/cli/commit/85cc35200ea044e9aabf1a9525f41344f755d156))
* build ffi script ([#320](https://github.com/rivet-gg/cli/issues/320)) ([3714ca0](https://github.com/rivet-gg/cli/commit/3714ca0ff2d5f6ab0cef0cde4c8d7f601d1a9045))
* **deploy:** update environment variables inline with deploy ([#298](https://github.com/rivet-gg/cli/issues/298)) ([65ee4cf](https://github.com/rivet-gg/cli/commit/65ee4cf74920e42be29101332d2fd3758f956534))
* only publish port for backend ([#323](https://github.com/rivet-gg/cli/issues/323)) ([ab3fc3e](https://github.com/rivet-gg/cli/commit/ab3fc3eb5fc8672b202a6258b0ac93cd258288c1))
* remove tty for docker runs ([#322](https://github.com/rivet-gg/cli/issues/322)) ([3b52607](https://github.com/rivet-gg/cli/commit/3b52607b60b1370a6173c30c85ca7537ba0990c7))


### Chores

* always set env vars ([#328](https://github.com/rivet-gg/cli/issues/328)) ([202244a](https://github.com/rivet-gg/cli/commit/202244a026feb3ad563473dcba6f67406595d30e))
* **backend:** add postgres mount for container ([#329](https://github.com/rivet-gg/cli/issues/329)) ([58ef34a](https://github.com/rivet-gg/cli/commit/58ef34abd0869d020c66dda8aed46eeab02a7a53))
* clean warnings ([#312](https://github.com/rivet-gg/cli/issues/312)) ([1208299](https://github.com/rivet-gg/cli/commit/1208299f61ac3ddc336d0770e7de96c3ce36a45e))
* disable unneeded api calls on deploy ([#333](https://github.com/rivet-gg/cli/issues/333)) ([243306a](https://github.com/rivet-gg/cli/commit/243306acdf40869ef2fa7f10a076f8b8fa354de7))
* expose game id & env id to backend ([#317](https://github.com/rivet-gg/cli/issues/317)) ([ff00497](https://github.com/rivet-gg/cli/commit/ff00497cf95c8839b5e748fe36e8f75da8be2345))
* fix show_term on windows ([#335](https://github.com/rivet-gg/cli/issues/335)) ([08be0ef](https://github.com/rivet-gg/cli/commit/08be0ef9966c66e582d7b51896cf8e1a433695ce))
* re-enable backend migrations ([#324](https://github.com/rivet-gg/cli/issues/324)) ([b9596c8](https://github.com/rivet-gg/cli/commit/b9596c8a3d7743f8ad72315c62e61722341ef099))
* release 2.0.0-rc.4 ([0aa270b](https://github.com/rivet-gg/cli/commit/0aa270b94dd80ea0b9721f24c001c81abbd6444e))
* rename opengb -&gt; backend in output ([#325](https://github.com/rivet-gg/cli/issues/325)) ([09bf73c](https://github.com/rivet-gg/cli/commit/09bf73ca5152061b543cb275fdcacd39b1c35b63))
* **settings:** disallow unknown props ([#316](https://github.com/rivet-gg/cli/issues/316)) ([d81d7a6](https://github.com/rivet-gg/cli/commit/d81d7a6ad1df655d263b65c3ab9f2111c7f375f7))
* update api ([#314](https://github.com/rivet-gg/cli/issues/314)) ([87255d4](https://github.com/rivet-gg/cli/commit/87255d40730b8cc2d5c926ed85a2b5e49c78decb))
* update build tags ([#318](https://github.com/rivet-gg/cli/issues/318)) ([48ae79f](https://github.com/rivet-gg/cli/commit/48ae79f220022e70a911627451fc4ebdced98e44))
* update env & backend apis ([#313](https://github.com/rivet-gg/cli/issues/313)) ([0204f9f](https://github.com/rivet-gg/cli/commit/0204f9fd6a6d50a107878f362b05d370c76eeef8))
* update opengb ([#334](https://github.com/rivet-gg/cli/issues/334)) ([d30fbba](https://github.com/rivet-gg/cli/commit/d30fbba326bc44699912966c9f909cf2cd715a68))
* update sdks ([#327](https://github.com/rivet-gg/cli/issues/327)) ([8f43c6b](https://github.com/rivet-gg/cli/commit/8f43c6b77f5261381dd352f77af8029d8ef309f1))
* update sdks ([#336](https://github.com/rivet-gg/cli/issues/336)) ([6cc7542](https://github.com/rivet-gg/cli/commit/6cc754201ed6fd0f6e6d89745c9d3e98a80828b0))

## [2.0.0-rc.3](https://github.com/rivet-gg/cli/compare/v2.0.0-rc.2...v2.0.0-rc.3) (2024-08-12)


### Bug Fixes

* clean opengb container after run ([#307](https://github.com/rivet-gg/cli/issues/307)) ([e98fd16](https://github.com/rivet-gg/cli/commit/e98fd16917bfdc669297adcf519a45525c7af227))
* compilation errors ([#306](https://github.com/rivet-gg/cli/issues/306)) ([647b087](https://github.com/rivet-gg/cli/commit/647b087ba8de1bcc7942310bce67c49c6f51c275))
* envfile race condition ([#305](https://github.com/rivet-gg/cli/issues/305)) ([8fda7e2](https://github.com/rivet-gg/cli/commit/8fda7e2b6799c992948932b9173f19b0582ce468))
* pass arguments to opengb in docker correctly ([#309](https://github.com/rivet-gg/cli/issues/309)) ([281ad54](https://github.com/rivet-gg/cli/commit/281ad54c071bdf1fdc0225b3376ceaadba68f32e))
* update opengb docker tag ([#311](https://github.com/rivet-gg/cli/issues/311)) ([ae4b4f6](https://github.com/rivet-gg/cli/commit/ae4b4f69bd87cda30b9f22d5d7b077b5515005f0))


### Continuous Integration

* update format check ([#308](https://github.com/rivet-gg/cli/issues/308)) ([7d2d208](https://github.com/rivet-gg/cli/commit/7d2d208b334a444f05b1dafa1179de606b254ef5))


### Chores

* release 2.0.0-rc.3 ([e0f72b5](https://github.com/rivet-gg/cli/commit/e0f72b5a33706201ba46f676d26d112701dc51d9))

## [2.0.0-rc.2](https://github.com/rivet-gg/cli/compare/v2.0.0-rc.1...v2.0.0-rc.2) (2024-08-10)


### Continuous Integration

* fix cargo-dist ([#300](https://github.com/rivet-gg/cli/issues/300)) ([ef5edc6](https://github.com/rivet-gg/cli/commit/ef5edc61f1d212357d7790107365a464eb2886fb))


### Chores

* release 2.0.0-rc.2 ([6dc6edf](https://github.com/rivet-gg/cli/commit/6dc6edff3670bd50e85543082b3130196a6a1e20))

## [2.0.0-rc.1](https://github.com/rivet-gg/cli/compare/v1.3.4...v2.0.0-rc.1) (2024-08-09)


### Features

* add --skip-migrate flag to opengb deploy ([#262](https://github.com/rivet-gg/cli/issues/262)) ([6255e07](https://github.com/rivet-gg/cli/commit/6255e0770dcea42d97b26357559fd0672257e675))
* add ffi ([#291](https://github.com/rivet-gg/cli/issues/291)) ([ffd015b](https://github.com/rivet-gg/cli/commit/ffd015b3cc711bef5b533e3d002f6d79c38cd26e))
* add opengb db command passthrough ([#216](https://github.com/rivet-gg/cli/issues/216)) ([7b78870](https://github.com/rivet-gg/cli/commit/7b788705687bd98387380e785614dbcc8c1190dd))
* add passthrough env var ([#231](https://github.com/rivet-gg/cli/issues/231)) ([2fc3021](https://github.com/rivet-gg/cli/commit/2fc30210e63e0230f88c9a7e04b54a66bb385fab))
* add settings file ([#278](https://github.com/rivet-gg/cli/issues/278)) ([c004d05](https://github.com/rivet-gg/cli/commit/c004d05e7255713a402b4d049b62615d1d14dfd2))
* add sidekick exec command ([#275](https://github.com/rivet-gg/cli/issues/275)) ([3121f94](https://github.com/rivet-gg/cli/commit/3121f940e4955e3cfa5772b8f98486f1ad323064))
* add sidekick show-term command ([#266](https://github.com/rivet-gg/cli/issues/266)) ([5fc1c89](https://github.com/rivet-gg/cli/commit/5fc1c89c3205f93411fd672934dbebf7bb1306ff))
* add support for sh and url db commands ([#217](https://github.com/rivet-gg/cli/issues/217)) ([bbeeaba](https://github.com/rivet-gg/cli/commit/bbeeaba7245839047c02f1f461869ab8c434e0ba))
* allow configuring opengb config path ([#283](https://github.com/rivet-gg/cli/issues/283)) ([aa8c212](https://github.com/rivet-gg/cli/commit/aa8c21284855489e2548be803e0e68a12ceeaf3e))
* **cli:** add config command ([#296](https://github.com/rivet-gg/cli/issues/296)) ([3787d0c](https://github.com/rivet-gg/cli/commit/3787d0c49d013477942f424c23f684a2f0266f7c))
* get lobby and logs links in sidekick ([#235](https://github.com/rivet-gg/cli/issues/235)) ([7c63efd](https://github.com/rivet-gg/cli/commit/7c63efd86a2ca4a659b0df8c88b3764f904f2938))
* Implement OpenGB related commands ([#215](https://github.com/rivet-gg/cli/issues/215)) ([ce57364](https://github.com/rivet-gg/cli/commit/ce57364d138d80ea48902733df1b3f796d51cd05))
* run opengb using docker by default ([#254](https://github.com/rivet-gg/cli/issues/254)) ([598ce8d](https://github.com/rivet-gg/cli/commit/598ce8da485ac035a834ab74bae33701b34af226))
* **sidekick:** add backend dev command ([#274](https://github.com/rivet-gg/cli/issues/274)) ([7547384](https://github.com/rivet-gg/cli/commit/75473841fb6fa2780f50c5daea6f86daeca730f7))
* **sidekick:** add backend gen command ([#273](https://github.com/rivet-gg/cli/issues/273)) ([2f1358e](https://github.com/rivet-gg/cli/commit/2f1358e9f857fec27b3310ee6357ba369af583d2))


### Bug Fixes

* add concurrency constraint to generated github action ([#226](https://github.com/rivet-gg/cli/issues/226)) ([8a62d97](https://github.com/rivet-gg/cli/commit/8a62d97bcea701983df02502f801d4ca8f403eef))
* **backend:** check opengb and deno installation using which crate ([#237](https://github.com/rivet-gg/cli/issues/237)) ([64b3489](https://github.com/rivet-gg/cli/commit/64b3489f61206f58299cff59a5583c45b4663bac))
* cdn.build_env not working ([#208](https://github.com/rivet-gg/cli/issues/208)) ([214fe29](https://github.com/rivet-gg/cli/commit/214fe297e612f6e88d06df7f57041be06f44949d))
* **ci:** update ci script to use json-compact instead of json ([#224](https://github.com/rivet-gg/cli/issues/224)) ([2f04ea3](https://github.com/rivet-gg/cli/commit/2f04ea3c0639065a10f4b2ecbf4cfc2bf587f353))
* disable hyper connection pooling ([#293](https://github.com/rivet-gg/cli/issues/293)) ([bddfa6d](https://github.com/rivet-gg/cli/commit/bddfa6dc996c42b476f9c8cc301a0cfd5981440f))
* **opengb:** one db per env ([#256](https://github.com/rivet-gg/cli/issues/256)) ([a3c4e10](https://github.com/rivet-gg/cli/commit/a3c4e109d6ec316fc72c296b97efa4ef1aef11f9))
* prevent asking user for terminal permissions ([#236](https://github.com/rivet-gg/cli/issues/236)) ([a1a75d8](https://github.com/rivet-gg/cli/commit/a1a75d858a99ab3830ed14917a26e8f06f446c4f))
* read_generated_manifest fn name ([#241](https://github.com/rivet-gg/cli/issues/241)) ([72970c7](https://github.com/rivet-gg/cli/commit/72970c7240f1dfa19a4fb75a9e009e8bde3799b5))
* reading byte-order marks on Windows ([#238](https://github.com/rivet-gg/cli/issues/238)) ([e177ad4](https://github.com/rivet-gg/cli/commit/e177ad4917945f6c99b8cd2f03c35bec3ba91941))
* revert regression with config-rs dependency ([#270](https://github.com/rivet-gg/cli/issues/270)) ([ff3afa8](https://github.com/rivet-gg/cli/commit/ff3afa8d12b1743b20365103b95fece74b4d1a39))
* rivet exec does not work with --rivet-servers ([#220](https://github.com/rivet-gg/cli/issues/220)) ([c1d33c5](https://github.com/rivet-gg/cli/commit/c1d33c5e251d29edd270d9a84a05d37fe39357ee))
* show term args on linux ([#286](https://github.com/rivet-gg/cli/issues/286)) ([6e94ab5](https://github.com/rivet-gg/cli/commit/6e94ab594f320a78fd6a57f1f2b8d33d30723969))
* temp disable flakey macos test ([#212](https://github.com/rivet-gg/cli/issues/212)) ([6d1ed9b](https://github.com/rivet-gg/cli/commit/6d1ed9b87b7aa9cdc249a895c7d5ea47ef9d5be7))
* **test:** update region names ([#223](https://github.com/rivet-gg/cli/issues/223)) ([c605561](https://github.com/rivet-gg/cli/commit/c605561bdf4b4206b0f367f7dc5e716f1f0f5f76))
* update cargo-release version ([450d25b](https://github.com/rivet-gg/cli/commit/450d25b060da494a6ec44990b7575772e765db3e))
* update sdks for opengb ([#233](https://github.com/rivet-gg/cli/issues/233)) ([7feb70b](https://github.com/rivet-gg/cli/commit/7feb70b2056d96ac31a69102d8a172ad6c0e0905))
* update sentry issue url ([#210](https://github.com/rivet-gg/cli/issues/210)) ([2b928df](https://github.com/rivet-gg/cli/commit/2b928dfc38f18e7f33865bf8c76614c88c8ce384))
* **upload:** increase upload buffer size ([#229](https://github.com/rivet-gg/cli/issues/229)) ([28d9d93](https://github.com/rivet-gg/cli/commit/28d9d93a9e7d6df959fa2a731c7433febfbe47b0))


### Documentation

* release script instructions ([#248](https://github.com/rivet-gg/cli/issues/248)) ([0d9edb3](https://github.com/rivet-gg/cli/commit/0d9edb3737989709ad9d3221d13c5471f997e6e2))


### Code Refactoring

* move global config to meta config ([#279](https://github.com/rivet-gg/cli/issues/279)) ([8ae2ed1](https://github.com/rivet-gg/cli/commit/8ae2ed1ecf69eaac7f044cdddd270883906152a2))
* move shared functionality to toolchain ([#277](https://github.com/rivet-gg/cli/issues/277)) ([d479186](https://github.com/rivet-gg/cli/commit/d479186499db118a4aef2d54bd7cb64ab4581187))


### Continuous Integration

* and release please pr ([#244](https://github.com/rivet-gg/cli/issues/244)) ([9862c5a](https://github.com/rivet-gg/cli/commit/9862c5ada4f935d64cc457d0ecd760a6d7d252b0))
* bring back release please ([#299](https://github.com/rivet-gg/cli/issues/299)) ([730b694](https://github.com/rivet-gg/cli/commit/730b69491dc8fd981a2248502b9c5780ce201a47))
* change label update to merged PRs ([#249](https://github.com/rivet-gg/cli/issues/249)) ([06a938c](https://github.com/rivet-gg/cli/commit/06a938cafedb2ed794358b0a9a370453fd2859dd))
* change release-please pr labels on release ([#247](https://github.com/rivet-gg/cli/issues/247)) ([336f789](https://github.com/rivet-gg/cli/commit/336f789b3909392fe92180ba75382f12d005c8de))
* explicitly fmt check members ([#242](https://github.com/rivet-gg/cli/issues/242)) ([f14b17e](https://github.com/rivet-gg/cli/commit/f14b17ed23a33b34f738975489a92d431dae1c59))
* ignore failing e2e test ([#243](https://github.com/rivet-gg/cli/issues/243)) ([242e291](https://github.com/rivet-gg/cli/commit/242e291db0febec9f70632536d1da39c1293ff30))


### Chores

* add back cli ([#288](https://github.com/rivet-gg/cli/issues/288)) ([9ca98e4](https://github.com/rivet-gg/cli/commit/9ca98e49c9536515dbd434e853f846f19d8af0e0))
* add back loading config ([#280](https://github.com/rivet-gg/cli/issues/280)) ([06347e1](https://github.com/rivet-gg/cli/commit/06347e168367e26cfcbe00264b2d8d4b9f23bbba))
* add deploy command ([#294](https://github.com/rivet-gg/cli/issues/294)) ([e9bffdb](https://github.com/rivet-gg/cli/commit/e9bffdbf163e70c39c5357d6f95321edfa791f91))
* add show_term task ([#289](https://github.com/rivet-gg/cli/issues/289)) ([72b37e3](https://github.com/rivet-gg/cli/commit/72b37e376d2d6db838c8b148d67c0089200fe8de))
* Bump the cargo group across 1 directory with 4 updates ([#228](https://github.com/rivet-gg/cli/issues/228)) ([a192e35](https://github.com/rivet-gg/cli/commit/a192e35aa5d5076be10d0f3b23836cfcc28ad1b0))
* bump version ([71c215f](https://github.com/rivet-gg/cli/commit/71c215febeebff27fad0cac1938e4e6663a12cdd))
* check system requirements ([#282](https://github.com/rivet-gg/cli/issues/282)) ([7858080](https://github.com/rivet-gg/cli/commit/7858080b62def9edfcf90fcc2d9632cae8816781))
* choose free port for opengb ([#281](https://github.com/rivet-gg/cli/issues/281)) ([1601f55](https://github.com/rivet-gg/cli/commit/1601f55e8729802940f1c09614c850b2ee846e34))
* default RunConfig will print to stdout ([#290](https://github.com/rivet-gg/cli/issues/290)) ([aa7de1a](https://github.com/rivet-gg/cli/commit/aa7de1a3c4d2eec9aa768ae45a5f1774bf3607ad))
* fmt sdk ([#251](https://github.com/rivet-gg/cli/issues/251)) ([28eade7](https://github.com/rivet-gg/cli/commit/28eade7f45a032f08d3fb9c55ddc3b3eda3a1be5))
* format ([#297](https://github.com/rivet-gg/cli/issues/297)) ([8ba0ec6](https://github.com/rivet-gg/cli/commit/8ba0ec60adcd24187e8f74838533dace65aac618))
* **main:** release 1.2.0 ([555fec1](https://github.com/rivet-gg/cli/commit/555fec1a2e1dacc08bc03cbfaed733f146d06220))
* **main:** release 1.3.0 ([#246](https://github.com/rivet-gg/cli/issues/246)) ([cfa546c](https://github.com/rivet-gg/cli/commit/cfa546cf84ca89f7234220f030e9080a1197abb2))
* **main:** release 1.3.1 ([#250](https://github.com/rivet-gg/cli/issues/250)) ([9a76764](https://github.com/rivet-gg/cli/commit/9a76764f9e832736e036a3fb3bdb1b6efdad3b5b))
* **main:** release 1.3.2 ([#255](https://github.com/rivet-gg/cli/issues/255)) ([e232c5f](https://github.com/rivet-gg/cli/commit/e232c5f0d41b7c9d33d745c3737c008c29d45803))
* **main:** release 1.3.3 ([#257](https://github.com/rivet-gg/cli/issues/257)) ([3d1c35f](https://github.com/rivet-gg/cli/commit/3d1c35f3fe24d0efd7ff50e9ca2bbe063cb2da31))
* **main:** release 1.3.4 ([#272](https://github.com/rivet-gg/cli/issues/272)) ([3d5d757](https://github.com/rivet-gg/cli/commit/3d5d7574b6e628c968d4be91a1db9bc6e5ed9c97))
* **opengb:** migrate from backend.yaml -&gt; backend.json ([#253](https://github.com/rivet-gg/cli/issues/253)) ([4b31887](https://github.com/rivet-gg/cli/commit/4b31887bd166e71958155b8cc5a75bc8246b6248))
* **readme:** add note about openssl when building from source ([#234](https://github.com/rivet-gg/cli/issues/234)) ([a9c1b29](https://github.com/rivet-gg/cli/commit/a9c1b295a4819fb88f9be5a0e52780d5ab92bf27))
* Release ([04d004e](https://github.com/rivet-gg/cli/commit/04d004efed9f6693c53eb3d0e5224476dadf8391))
* Release ([4951a00](https://github.com/rivet-gg/cli/commit/4951a001465ceeeab209978b47053b03d6e32ec1))
* Release ([48b18ba](https://github.com/rivet-gg/cli/commit/48b18baca3cf99cd80870bedeabe966bbb77b0cd))
* Release ([ab5e081](https://github.com/rivet-gg/cli/commit/ab5e081f3f1eb4ac12c9b54f90953318e640afb6))
* Release ([b10bc24](https://github.com/rivet-gg/cli/commit/b10bc2414434bc1a93690ea2948feb52003f4bcd))
* Release ([6c40a0e](https://github.com/rivet-gg/cli/commit/6c40a0ea758ba9e845539d2bfc64fcd1bf35b7a9))
* Release ([e9ff7fc](https://github.com/rivet-gg/cli/commit/e9ff7fc971969963427e9dc50600c3177bab54f0))
* Release ([db9e6ab](https://github.com/rivet-gg/cli/commit/db9e6abef659db20857bc500f2236323477a3f19))
* Release ([bcd1d34](https://github.com/rivet-gg/cli/commit/bcd1d3471364ff7a634133b1c0fc8b39594cf636))
* Release ([d1ac3c6](https://github.com/rivet-gg/cli/commit/d1ac3c69ddf1bb15f284da3886d47bcb9a39e232))
* Release ([d0e2d6b](https://github.com/rivet-gg/cli/commit/d0e2d6b8557099b203ceb13264dcff95f79b7f05))
* Release ([bd657c8](https://github.com/rivet-gg/cli/commit/bd657c868f727032d87d3c392adc6ff8709b5272))
* Release ([1a6bec0](https://github.com/rivet-gg/cli/commit/1a6bec0254b5c6a3f4bda2d690d1129f83bfe702))
* Release ([68819c3](https://github.com/rivet-gg/cli/commit/68819c36514e7d0cc370e0fbb451a5e0f1fbf80e))
* Release ([2f23b00](https://github.com/rivet-gg/cli/commit/2f23b009dca0947ef58f619413bc04540b3078c6))
* Release ([d057971](https://github.com/rivet-gg/cli/commit/d057971c08a872fa23ddffc2c126840ac37a86dd))
* Release ([260ea03](https://github.com/rivet-gg/cli/commit/260ea0354c13e4c1edd0381ab093dacad624d5e5))
* Release ([9350c01](https://github.com/rivet-gg/cli/commit/9350c01ee444ce78570f846ffb6c2531cb7dbfe1))
* Release ([dfe5722](https://github.com/rivet-gg/cli/commit/dfe5722b1bea9d3ce483230f9c1c8db265fc9e5b))
* Release ([2e6508c](https://github.com/rivet-gg/cli/commit/2e6508ccdecfe204794022068c25f62e117c0f8f))
* Release ([dc96983](https://github.com/rivet-gg/cli/commit/dc969832d6049b27fc1e3b5a81729c59f6d3be17))
* release 1.3.2 ([50bd2be](https://github.com/rivet-gg/cli/commit/50bd2be003cf8561f5d444a6abd611e3f1078af5))
* release 1.3.3 ([aed2a03](https://github.com/rivet-gg/cli/commit/aed2a039b01d27c96008f9fbc82c8b25251e2eb8))
* release 2.0.0-rc.1 ([8882ad8](https://github.com/rivet-gg/cli/commit/8882ad88d8b50422fabdb732ebb96aed3e7940c1))
* remove experimental flag from run & exec commands ([#222](https://github.com/rivet-gg/cli/issues/222)) ([c9cfae6](https://github.com/rivet-gg/cli/commit/c9cfae60971e465c7fde5e50be654daa49a5abfd))
* rename --rivet-servers to --server & --this-machine to --dev ([#221](https://github.com/rivet-gg/cli/issues/221)) ([0f47917](https://github.com/rivet-gg/cli/commit/0f479176a496c123035eb679c68d164fdbbdb354))
* reorg package in prep for toolchain ([#276](https://github.com/rivet-gg/cli/issues/276)) ([371ac16](https://github.com/rivet-gg/cli/commit/371ac16c28e293b7f46f066c16e53dcf9b233eae))
* return backend project on bootstrap ([#284](https://github.com/rivet-gg/cli/issues/284)) ([40ea899](https://github.com/rivet-gg/cli/commit/40ea8998935afda712bba36cddf9b4297538c140))
* Update cargo dist ([#271](https://github.com/rivet-gg/cli/issues/271)) ([c2355ae](https://github.com/rivet-gg/cli/commit/c2355ae705fea6a1bd007412b7628758ccf1a78e))
* update docker root user help link ([#214](https://github.com/rivet-gg/cli/issues/214)) ([30fdc56](https://github.com/rivet-gg/cli/commit/30fdc56adb2c686496a7ded0406c2bb2255691d5))
* update opengb flag --path -&gt; --project ([#295](https://github.com/rivet-gg/cli/issues/295)) ([6b5407e](https://github.com/rivet-gg/cli/commit/6b5407e5f0b03cfcda22bb3d9640cac52aa82ecb))
* update posthog api key ([#263](https://github.com/rivet-gg/cli/issues/263)) ([5f2ee58](https://github.com/rivet-gg/cli/commit/5f2ee58a5b5bb204dd7d4178446bb1de304a9c0f))
* update rivet-api ([#209](https://github.com/rivet-gg/cli/issues/209)) ([b057d00](https://github.com/rivet-gg/cli/commit/b057d00c0fc897b9f663a55c4aff8845685745c4))
* update sdk for env endpoint ([#285](https://github.com/rivet-gg/cli/issues/285)) ([73c7120](https://github.com/rivet-gg/cli/commit/73c7120e08da4881fa901bc153f7a77b31cf4262))

## [1.3.4](https://github.com/rivet-gg/cli/compare/v1.3.3...v1.3.4) (2024-07-11)


### Chores

* Update cargo dist ([#271](https://github.com/rivet-gg/cli/issues/271)) ([c2355ae](https://github.com/rivet-gg/cli/commit/c2355ae705fea6a1bd007412b7628758ccf1a78e))

## [1.3.3](https://github.com/rivet-gg/cli/compare/v1.3.2...v1.3.3) (2024-07-10)


### Features

* add --skip-migrate flag to opengb deploy ([#262](https://github.com/rivet-gg/cli/issues/262)) ([6255e07](https://github.com/rivet-gg/cli/commit/6255e0770dcea42d97b26357559fd0672257e675))


### Bug Fixes

* **opengb:** one db per env ([#256](https://github.com/rivet-gg/cli/issues/256)) ([a3c4e10](https://github.com/rivet-gg/cli/commit/a3c4e109d6ec316fc72c296b97efa4ef1aef11f9))
* revert regression with config-rs dependency ([#270](https://github.com/rivet-gg/cli/issues/270)) ([2dbb0b9](https://github.com/rivet-gg/cli/commit/2dbb0b9871e3bd8e342ab48f08dc8104677df1b7))


### Chores

* release 1.3.3 ([aae8e3b](https://github.com/rivet-gg/cli/commit/aae8e3bd2e58baa4fc75888b40ecf1a194077205))
* update posthog api key ([#263](https://github.com/rivet-gg/cli/issues/263)) ([5f2ee58](https://github.com/rivet-gg/cli/commit/5f2ee58a5b5bb204dd7d4178446bb1de304a9c0f))

## [1.3.2](https://github.com/rivet-gg/cli/compare/v1.3.1...v1.3.2) (2024-06-13)


### Features

* run opengb using docker by default ([#254](https://github.com/rivet-gg/cli/issues/254)) ([598ce8d](https://github.com/rivet-gg/cli/commit/598ce8da485ac035a834ab74bae33701b34af226))


### Chores

* release 1.3.2 ([50bd2be](https://github.com/rivet-gg/cli/commit/50bd2be003cf8561f5d444a6abd611e3f1078af5))

## [1.3.1](https://github.com/rivet-gg/cli/compare/v1.3.0...v1.3.1) (2024-06-10)


### Continuous Integration

* change label update to merged PRs ([#249](https://github.com/rivet-gg/cli/issues/249)) ([06a938c](https://github.com/rivet-gg/cli/commit/06a938cafedb2ed794358b0a9a370453fd2859dd))


### Chores

* fmt sdk ([#251](https://github.com/rivet-gg/cli/issues/251)) ([28eade7](https://github.com/rivet-gg/cli/commit/28eade7f45a032f08d3fb9c55ddc3b3eda3a1be5))
* **opengb:** migrate from backend.yaml -&gt; backend.json ([#253](https://github.com/rivet-gg/cli/issues/253)) ([4b31887](https://github.com/rivet-gg/cli/commit/4b31887bd166e71958155b8cc5a75bc8246b6248))
* **readme:** add note about openssl when building from source ([#234](https://github.com/rivet-gg/cli/issues/234)) ([a9c1b29](https://github.com/rivet-gg/cli/commit/a9c1b295a4819fb88f9be5a0e52780d5ab92bf27))

## [1.3.0](https://github.com/rivet-gg/cli/compare/v1.2.0...v1.3.0) (2024-05-29)


### Features

* get lobby and logs links in sidekick ([#235](https://github.com/rivet-gg/cli/issues/235)) ([7c63efd](https://github.com/rivet-gg/cli/commit/7c63efd86a2ca4a659b0df8c88b3764f904f2938))


### Bug Fixes

* prevent asking user for terminal permissions ([#236](https://github.com/rivet-gg/cli/issues/236)) ([a1a75d8](https://github.com/rivet-gg/cli/commit/a1a75d858a99ab3830ed14917a26e8f06f446c4f))
* read_generated_manifest fn name ([#241](https://github.com/rivet-gg/cli/issues/241)) ([72970c7](https://github.com/rivet-gg/cli/commit/72970c7240f1dfa19a4fb75a9e009e8bde3799b5))
* reading byte-order marks on Windows ([#238](https://github.com/rivet-gg/cli/issues/238)) ([e177ad4](https://github.com/rivet-gg/cli/commit/e177ad4917945f6c99b8cd2f03c35bec3ba91941))


### Documentation

* release script instructions ([#248](https://github.com/rivet-gg/cli/issues/248)) ([0d9edb3](https://github.com/rivet-gg/cli/commit/0d9edb3737989709ad9d3221d13c5471f997e6e2))


### Continuous Integration

* and release please pr ([#244](https://github.com/rivet-gg/cli/issues/244)) ([9862c5a](https://github.com/rivet-gg/cli/commit/9862c5ada4f935d64cc457d0ecd760a6d7d252b0))
* change release-please pr labels on release ([#247](https://github.com/rivet-gg/cli/issues/247)) ([336f789](https://github.com/rivet-gg/cli/commit/336f789b3909392fe92180ba75382f12d005c8de))
* explicitly fmt check members ([#242](https://github.com/rivet-gg/cli/issues/242)) ([f14b17e](https://github.com/rivet-gg/cli/commit/f14b17ed23a33b34f738975489a92d431dae1c59))
* ignore failing e2e test ([#243](https://github.com/rivet-gg/cli/issues/243)) ([242e291](https://github.com/rivet-gg/cli/commit/242e291db0febec9f70632536d1da39c1293ff30))


### Chores

* Bump the cargo group across 1 directory with 4 updates ([#228](https://github.com/rivet-gg/cli/issues/228)) ([a192e35](https://github.com/rivet-gg/cli/commit/a192e35aa5d5076be10d0f3b23836cfcc28ad1b0))
* **main:** release 1.2.0 ([555fec1](https://github.com/rivet-gg/cli/commit/555fec1a2e1dacc08bc03cbfaed733f146d06220))

## [1.2.0](https://github.com/rivet-gg/cli/compare/v1.1.0...v1.2.0) (2024-05-28)


### Features

* add opengb db command passthrough ([#216](https://github.com/rivet-gg/cli/issues/216)) ([7b78870](https://github.com/rivet-gg/cli/commit/7b788705687bd98387380e785614dbcc8c1190dd))
* add passthrough env var ([#231](https://github.com/rivet-gg/cli/issues/231)) ([2fc3021](https://github.com/rivet-gg/cli/commit/2fc30210e63e0230f88c9a7e04b54a66bb385fab))
* add support for sh and url db commands ([#217](https://github.com/rivet-gg/cli/issues/217)) ([bbeeaba](https://github.com/rivet-gg/cli/commit/bbeeaba7245839047c02f1f461869ab8c434e0ba))
* Implement OpenGB related commands ([#215](https://github.com/rivet-gg/cli/issues/215)) ([ce57364](https://github.com/rivet-gg/cli/commit/ce57364d138d80ea48902733df1b3f796d51cd05))


### Bug Fixes

* add concurrency constraint to generated github action ([#226](https://github.com/rivet-gg/cli/issues/226)) ([8a62d97](https://github.com/rivet-gg/cli/commit/8a62d97bcea701983df02502f801d4ca8f403eef))
* **backend:** check opengb and deno installation using which crate ([#237](https://github.com/rivet-gg/cli/issues/237)) ([64b3489](https://github.com/rivet-gg/cli/commit/64b3489f61206f58299cff59a5583c45b4663bac))
* **ci:** update ci script to use json-compact instead of json ([#224](https://github.com/rivet-gg/cli/issues/224)) ([2f04ea3](https://github.com/rivet-gg/cli/commit/2f04ea3c0639065a10f4b2ecbf4cfc2bf587f353))
* read_generated_manifest fn name ([#241](https://github.com/rivet-gg/cli/issues/241)) ([72970c7](https://github.com/rivet-gg/cli/commit/72970c7240f1dfa19a4fb75a9e009e8bde3799b5))
* update sdks for opengb ([#233](https://github.com/rivet-gg/cli/issues/233)) ([7feb70b](https://github.com/rivet-gg/cli/commit/7feb70b2056d96ac31a69102d8a172ad6c0e0905))
* **upload:** increase upload buffer size ([#229](https://github.com/rivet-gg/cli/issues/229)) ([28d9d93](https://github.com/rivet-gg/cli/commit/28d9d93a9e7d6df959fa2a731c7433febfbe47b0))


### Continuous Integration

* and release please pr ([#244](https://github.com/rivet-gg/cli/issues/244)) ([9862c5a](https://github.com/rivet-gg/cli/commit/9862c5ada4f935d64cc457d0ecd760a6d7d252b0))
* explicitly fmt check members ([#242](https://github.com/rivet-gg/cli/issues/242)) ([f14b17e](https://github.com/rivet-gg/cli/commit/f14b17ed23a33b34f738975489a92d431dae1c59))
* ignore failing e2e test ([#243](https://github.com/rivet-gg/cli/issues/243)) ([242e291](https://github.com/rivet-gg/cli/commit/242e291db0febec9f70632536d1da39c1293ff30))


### Chores

* Release ([b10bc24](https://github.com/rivet-gg/cli/commit/b10bc2414434bc1a93690ea2948feb52003f4bcd))

## [v1.1.0] - 2024-04-13

### Added

- `rivet run` and `rivet exec` are no longer experimental

### Changed

- Rename `--rivet-servers` to `--servers` and `--this-machine` to `--dev` for `rivet run` and `rivet exec`

### Fixed

- `rivet exec` does not respect `--rivet-servers` flag

## [v1.0.2] - 2024-02-29

### Changed

- Progress bars will consolidate to 1 if there are more than 40 files being uploaded
- Update SDKs

### Fixed

- `cdn.build_env` not being passed to `cdn.build_cmd`

## [v1.0.1] - 2024-01-29

### Changed

- Improved progress indicators on file uploads

### Fixed

- Docker image UID & GID validation not getting ran
- Lack of a newline printed by `rivet token create` causing EOL mark to appear on zsh shells

## [v1.0.0] - 2024-01-23

## [v1.0.0-rc.3] - 2024-01-19

### Added

- Shorthand API endpoints can now be passed without the scheme (e.g. `api.mydomain.com` or `127.0.0.1:8080`)
- `rivet global-config read-project` command
- `rivet global-config path` command to get the path to the global config
- `--format` now supports `json-compact`

### Changed

- `--format json` now defaults to pretty-printed JSON

### Fixed

- `rivet unlink` now works even if the credentials are invalid
- Docker image UID & GID validation no longer disabled by default

## [v1.0.0-rc.2] - 2024-01-13

### Added

- `rivet exec` command to run arbitrary commands with `RIVET_API_ENDPOINT` and `RIVET_TOKEN` environment variables
- `rivet run` command to run scripts from the `scripts` portion of `rivet.yaml` with `RIVET_API_ENDPOINT`, `RIVET_TOKEN`, and `RIVET_NAMESPACE` environment variables
- `rivet deploy` now can now specify the namespace inline (e.g. `rivet deploy prod` instead of `rivet deploy -n prod`)
- `matchmaker.docker.build_args` to configure Docker build args
- `cdn.build_env` to configure environment variables for building the site
- `RIVET_API_ENDPOINT` and `RIVET_NAMESPACE` arg is passed to `docker build` by default
- `RIVET_TOKEN` and `RIVET_NAMESPACE` now additionally passed to `cdn.build_command`

### Changed

- Reworked `rivet init` process to cleanly communicate next steps & unique links for the selected engine
- Updated generated `rivet.yaml` on `rivet init` to be more concise and helpful & unique content for the selected engine
- Update OCI bundle archival process to operate on TAR streams instead of using the host's file system to preserve ownership & permissions
- **[BREAKING]** `rivet deploy` now requires a `--no-namespace` flag if no namespace is provided

### Fixed

- Overriding `matchmaker.docker.image_id` getting ignored
- `rivet config validate` now uses `--print` flag instead of a positional argument
- Validate Docker images do not run as GID 0

## [v1.0.0-rc.1] - 2023-12-24

### Added

- Add `x86_64-unknown-linux-musl` artifact
- Version names are now generated with incrementing indexes on the backend without race conditions
- Warning if running unauthenticated commands as a sudo user
- `sidekick unlink` subcommand to unlink the current project from the
  Rivet CLI
- `sidekick generate-config` subcommand to generate a Rivet config file
- `sidekick get-namespace-dev-token` and `sidekick
get-namespace-public-token` subcommands to get a Rivet token for a namespace
- `sidekick get-bootstrap-data` subcommand to get the initial data about
  the signed-in user
- `sidekick get-cli-version` subcommand to get the version of the Rivet
  CLI
- `sidekick deploy` to do the process of deploying a build to Rivet
- ability for `sidekick` to open terminal in a new window for commands
  that need to be shown (e.g. `sidekick deploy`)
- `sidekick get-version` subcommand to get the manage version URL in the
  hub
- `sidekick get-token` subcommand to get a Rivet token for a user
- `sidekick check-login-state` subcommand to see if a user is logged in
  through the CLI
- `sidekick wait-for-login` subcommand to long-poll for a user to sign in
- `sidekick get-link` subcommand to get a sign-in link for a user
- hidden `Sidekick` subcommand to be used by external tools (e.g. engine
  plugins) to interact with the Rivet CLI

### Changed

- Cleaner unauthenticated error
- Changed `sidekick` to a more modular architecture
- Changed error handling in CLI to use `Result` from main repo instead of
  `anyhow`
- Unix install script can now take the environment variable `BIN_DIR` to specify
  the installation directory, preventing the need for sudo in certain cases
- Rivet CLI now references the `rivet-cli-api` from the Rivet main repo rather
  than storing its own copy
- Update `cargo-dist` to 0.6.2

### Fixed

- Custom engines no longer get prompted to select engine when running `rivet init` for the second time
- Windows compilation no longer fails with `nix` dependency
- `--telemetry-disabled` no longer requires explicit `true`
- Collect system metrics using `sysinfo::System` instead of `uname` command for compatability with Windows
- CDN URL on deploy complete now pulls dynamic DNS from bootstrap API
- CDN URL on deploy complete is no longer displayed if CDN is not enabled for the game

## [v0.4.0] - 2023-12-20

### Added

- Auto-generate GitHub Actions with `rivet ci generate github`
- Development token cache to make `rivet token create development` run faster
- Shorthand `-n` for `--namespace` flag in `rivet token create development`
- `rivet deploy` validates config before building & uploading resources
- `rivet unlink` command to remove authentication token
- Pretty-printed errors instead of default debug format
- Error reporting to Sentry

### Changed

- Removed engine prompt if Rivet config already exists
- **[BREAKING]** No longer automatically creates/updates `.env` file in favor of using `rivet token create development`
- Global flags (`--api-endpoint`, `--token`, and `--disable-telemetry`) can now be used in subcommands (e.g. `rivet init --token foobar` instead of `rivet --token foobar init`)
- Moved project metadata to global configuration file
- Removed `.rivet` from auto-generated `.gitignore`
- `rivet namespace create` can be called without specifying `--version`
- **[BREAKING]** Change `TELEMETRY_DISABLED` env var to `RIVET_TELEMETRY_DISABLED`
- Remove trailing line break from `rivet token create development`
- Rename `rivet site` subcommands to `rivet cdn` (alias still supported)
- Rename `rivet image` subcommands to `rivet docker` (alias still supported)
- Rename `dashboard` subcommands to `view` (alias still supported)
- Move `rivet version deploy` to `rivet deploy`
- Move `rivet version config-validate` to `rivet config validate`
- Move `RIVET_CONCURRENT_UPLOADS` env var to CLI flag on appropriate commands (env var still works)
- Streamline `rivet init` experience
- Add `rivet token create public` command

### Fixed

- Fix `matchmaker.game_modes.*.docker.image_id` falling back to `matchmaker.docker.image_id`
- **Install script** Now installs non-prerelease GitHub releases

## [v0.3.0] - 2023-12-10

### Added

- **Install script (Unix)** Configure installation directory by passing `$BIN_DIR`
- **Install script (Unix)** Warning if `$BIN_DIR` is not in `$PATH`

### Changed

- Auto-generated & recommended config is now a `rivet.yaml` file
- Default version names are now generated as `YYYY.MM (X)` format (where `X` is an incrementing index)
- Merged `.rivet/cloud_token` and `.rivet/config.toml` in to unified internal `.rivet/config.yaml` config file
- **[BREAKING]** Removed support for file formats that are not YAML, TOML, or JSON in order to simplify maintaining forward compatibility
- **[BREAKING]** Throw error if both `.yaml` and `.yml` config exist

### Fixed

- **Install script (Unix)** Installing ARM64 `jq` binary on ARM-based Macs
- **Install script (Unix)** Automatically create `$BIN_DIR` if doesn't exist, specifically on macOS Sonoma which does not provide a `/usr/local/bin` by default

## [v0.2.0] - 2023-12-1

### Added

- Support for building OCI bundles
- Support for LZ4 compression of builds
- **[BREAKING]** Expose `RIVET_API_ENDPOINT` to `cdn.build_command` to help automate deploying to multiple clusters
- **[BREAKING]** Unset `RIVET_TOKEN` to `cdn.build_command` in order to ensure the cloud token isn't accidentally baked in a build
- `image build-push` command to automatically build & push an image
- `site build-push` command to automatially build and push a site
- E2E cross-platform tests in GitHub Actions

### Changed

- **[BREAKING]** Support new single-origin API endpoint (configured with `RIVET_API_ENDPOINT` environment variable or `--api-endpoint` flag)
- **[BREAKING]** Rename `RIVET_CLOUD_TOKEN` environment variable to `RIVET_TOKEN`
- **[BREAKING]** Rename `--cloud-token` flag to `--token`
- **[BREAKING]** Removed `RIVET_API_CLOUD_URL` in favor of `RIVET_API_ENDPOINT`
- **[BREAKING]** Updated custom games config schema
- **[BREAKING]** Removed domain map from turnstile configuration, replaced with `site_key` and `secret_key`
- Added telemetry beacon for fatal errors. Opt out with `--telemetry-disabled` or `TELEMETRY_DISABLED=1`
- Added internal config to store api endpoint and telemetry options
- Implemented multipart uploads for builds and sites, disable multipart uploads with `_RIVET_UPLOAD_DISABLE_MULTIPART`

## [v0.1.4] - 2023-12-9

### Added

- Darwin ARM release artifact

### Changed

- Update `cargo-dist` to 0.5.0

## [v0.1.3] - 2023-12-3

### Changed

- Replace Smithy-generated API library with OpenAPI-generated library in order to fix `invalid certificate timestamp: UnknownLog` error

## [v0.1.2] - 2023-08-26

### Changed

- Added custom games + lobby state + external verification

## [v0.1.1] - 2023-07-17

### Changed

- `rivet deploy` now gracefully falls back to the native build method if Docker Buildx is not installed

## [v0.1.0] - 2023-07-17

### Added

- Unreal helper in `rivet init`
- Installer for the Unreal Engine plugin with `rivet unreal install-plugin` or `rivet init --unreal`

### Changed

- Renamed `rivet.version.toml` to `rivet.toml`. All changes are backwards compatible.
- Renamed `rivet publish` command to `rivet deploy` since this is the more commonly used alias
- `rivet token create dev` now prints token in plain text

### Fixed

- Broken links to old docs
- Docker builder now catches missing builder errors correctly for older Docker versions

## [v0.0.51] - 2023-04-26

### Fixed

- Docker builder now catches missing builder errors correctly for older Docker versions

### Changed

- Remove `PORT`, `RIVET_LOBBY_TOKEN`, and `RIVET_PUBLIC_TOKEN` from generated .env file
- Document development token in .env

## [v0.0.50] - 2023-04-18

### Changed

- Description, homepage, and repository to Cargo.toml

### Fixed

- Incorrect package version

## [v0.0.49] - 2023-04-18

### Added

- Experimental build configuration flag `_RIVET_DOCKER_BUILD_METHOD` can be set to `buildx` or `native`

### Changed

- Default Docker build method is now Buildx, even if the native platform is x86
- Update dependency: `rivet-api`
- Upgrade dependency: `tokio 1.27`
- Removed unnecessary feature flags from `tokio`
