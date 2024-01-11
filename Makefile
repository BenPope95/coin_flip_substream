ENDPOINT ?= mainnet.eth.streamingfast.io:443

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: stream
stream: build
	substreams run -e $(ENDPOINT) substreams.yaml graph_out -s 5382741 -t +500

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: package
package:
	substreams pack ./substreams.yaml
