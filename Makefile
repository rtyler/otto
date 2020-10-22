##
# This Makefile provides the main development interface for working with Otto,
# and helps organize the various tasks for preparation, compilation, and
# testing.
#
# Execute `make` to get help for the various targets
################################################################################

################################################################################
## Phony targets

release:
	cargo build --release
	# Strip all the executables for size, does impact debug symbols
	find target/release -type f -executable -exec strip {} \;

steps: release
	for dir in $$(find stdlib -maxdepth 1 -type d | tail -n +2); do \
		echo ">> Packaging $$dir"; \
		./target/release/osp $$dir; \
	done;

# Cute hack thanks to:
# https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
help: ## Display this help text
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

clean: ## Clean all temporary/working files

diagram: system.png system.dot ## Generate the diagrams describing otto
	dot -Tpng -o system.png system.dot

################################################################################

.PHONY: clean diagram help steps release
