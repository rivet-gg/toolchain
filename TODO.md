init
ns create --ignore-duplicate --config rivet.namespace.toml --override '{}'
ns set-version
ns open
version create --ignore-duplicate --ns xxxx --config rivet.version.toml --override '{"matchmaker":{"lobby_groups":{}}'
version set-ns
version open


(use config.rs library)
Rivet.toml
rivet.json

