# Actor Crash Course

## Prerequisites

- Must have Rivet docker compose running
- Must have hub running just for the device link

## Actor test

```bash
cargo build
cd examples/js-deno
../../target/debug/rivet login --api-endpoint http://localhost:8080
# copy the path to localhost:5080/device/link/... and finish linking
../../target/debug/rivet deploy default
# copy build id
../../target/debug/rivet actor create default -t name=rng --build MY_BUILD_ID --region local --network-mode host --port protocol=tcp,name=http,host
# copy public_hostname and public_port
curl 127.0.0.1:20081
../../target/debug/rivet actor destroy default --id MY_ACTOR_ID
```

## Reference

- See `rivet --help` for more commands
- rivet.jsonc config spec at `packages/toolchain/src/config/mod.rs`
- WIP typedefs for actors [here](https://github.com/rivet-gg/rivet/blob/925b9b5c2f024f40615e910e9670655249eb2bc7/sdks/actor-types/src/internal/90_rivet_ns.ts)

## Known issues

- Only supports host networking
- Networking is not working (afaik)
- LZ4 compression for JS tars don't work (defaults to correct compression, no action needed)

## Troubleshooting

### `Error getting bootstrap: get project failed`

Make sure to run `rivet logout` if you reset the core cluster.

