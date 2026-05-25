VERSION := $(shell cargo metadata --no-deps --format-version 1 | python3 -c "import sys,json; print(json.load(sys.stdin)['packages'][0]['version'])")

.PHONY: release
release:
	git tag v$(VERSION)
	git push origin v$(VERSION)
