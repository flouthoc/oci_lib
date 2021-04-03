install-node-deps:
	(npm install @apidevtools/json-schema-ref-parser)
	(npm install  quicktype-core)

sync:
	(cd image-spec; git pull https://github.com/opencontainers/image-spec)
	(cd runtime-spec; git pull https://github.com/opencontainers/runtime-spec)
	(rm -f src/runtime/mod.rs; rm -f src/image/mod.rs)
	(node generator.js)

