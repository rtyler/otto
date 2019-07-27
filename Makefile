##
# This Makefile provides the main development interface for working with Otto,
# and helps organize the various tasks for preparation, compilation, and
# testing.
#
# Execute `make` to get help ffor the various targets
################################################################################

# Set the PATH so we can automatically include our node binstubs
export PATH:=./node_modules/.bin:${PATH}
SUB_DIRS=grammar

################################################################################
## Phony targets
# Cute hack thanks to:
# https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
help: ## Display this help text
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

build: depends ## Build all components
	$(foreach dir, $(SUB_DIRS), $(MAKE) -C $(dir) $@)
	tsc

lint: depends
	tslint -c tslint.json -t stylish 'lib/**/*.ts' 'services/**/*.ts'

check: depends lint build ## Run validation tests
	jest
	dredd

swagger: depends ## Generate the swagger stubs based on apispecs

depends: prereqs ## Download all dependencies

prereqs: scripts/prereqs.sh ## Check that this system has the necessary tools to build otto
	@sh scripts/prereqs.sh

clean: ## Clean all temporary/working files
	$(foreach dir, $(SUB_DIRS), $(MAKE) -C $(dir) $@)

diagram: system.png system.dot ## Generate the diagrams describing otto
	dot -Tpng -o system.png system.dot

################################################################################

.PHONY: all build check clean depends lint swagger
