run:
	cargo run

check:
	cargo clippy

test:
	cargo test --tests

clean:
	rm -f .data/*.json use_elementwise_serialize/.data/*.json

.PHONY: check test clean