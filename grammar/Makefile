# List of .g4 files to compile
GRAMMAR=Otto.g4 OttoLexer.g4
# Target languages for the grammars
LANGS=JavaScript Java Cpp Go
# Antlr binary for execution
ANTLR_BIN=antlr-4.7.2-complete.jar
ANTLR=../contrib/$(ANTLR_BIN)

################################################################################
## Phony targets
################################################################################

# Cute hack thanks to:
# https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
help: ## Display this help text
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'


build: $(GRAMMAR) $(ANTLR) ## Compile the grammars into their language stubs
	@for target in $(LANGS); do \
		java -cp $(ANTLR) org.antlr.v4.Tool \
			-Dlanguage=$$target \
			-o build/parser/$$target \
			$(GRAMMAR); \
		echo "--> Generated $$target stubs"; \
	done;



clean: ## Clean all temporary/working files
	rm -f $(ANTLR)


.PHONY: help
################################################################################


$(ANTLR): ## Download the latest ANTLR4 binary
	wget -O $(ANTLR) https://www.antlr.org/download/$(ANTLR_BIN)
