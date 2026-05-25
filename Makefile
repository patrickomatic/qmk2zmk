VERSION := $(shell cargo metadata --no-deps --format-version 1 | python3 -c "import sys,json; print(json.load(sys.stdin)['packages'][0]['version'])")

.PHONY: release
release:
	cargo test
	cargo clippy --all-targets -- -W clippy::pedantic -D warnings
	git add Cargo.toml Cargo.lock
	git commit -m "Bump version to $(VERSION)"
	git push
	git tag v$(VERSION)
	git push origin v$(VERSION)
	cargo publish
