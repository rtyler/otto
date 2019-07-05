##
# This Makefile provides the main development interface for working with Otto,
# and helps organize the various tasks for preparation, compilation, and
# testing.
#
# Execute `make` to get help ffor the various targets
################################################################################

# Set the PATH so we can automatically include our node binstubs
export PATH:=./node_modules/.bin:${PATH}

ANTLR_BIN=antlr-4.7.2-complete.jar
ANTLR=contrib/$(ANTLR_BIN)
GRAMMAR=Otto.g4 OttoLexer.g4

################################################################################
## Phony targets
all: help

build: depends ## Build all components
	tsc

check: depends build ## Run validation tests
	#dredd
	node parse-test.js

swagger: depends ## Generate the swagger stubs based on apispecs

depends: prereqs $(ANTLR) ## Download all dependencies

prereqs: scripts/prereqs.sh ## Check that this system has the necessary tools to build otto
	@sh scripts/prereqs.sh

clean: ## Clean all temporary/working files
	rm -f $(ANTLR)

parser: depends $(GRAMMAR) ## Generate the parser code
	@for target in Java JavaScript; do \
		java -cp $(ANTLR) org.antlr.v4.Tool \
			-Dlanguage=$$target \
			-o build/parser/$$target \
			$(GRAMMAR); \
		echo "--> Generated $$target stubs"; \
	done;

################################################################################
## Non-phony targets
$(ANTLR): ## Download the latest ANTLR4 binary
	(cd contrib && wget https://www.antlr.org/download/$(ANTLR_BIN))

################################################################################

# Cute hack thanks to:
# https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
help: ## Display this help text
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: all build check clean depends parser
