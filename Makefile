test:
	cargo test --features api_test

docs:
	@cargo doc --no-deps

upload-docs: docs
	@./upload-docs.sh

.PHONY: test docs upload-docs
