.PHONY: help clean go today day_% inputs cog build
.DEFAULT_GOAL := today

.PRECIOUS: inputs/day_%.txt
.FORCE:

HOOKS=$(.git/hooks/pre-commit)

BINPATH=$(shell which python | xargs dirname | xargs realpath --relative-to=".")
COG_PATH:=$(BINPATH)/cog

ALLDAYS=$(wildcard src/day_*.py)
ALLINPUTS=$(subst src/,inputs/,$(subst .py,.txt,$(ALLDAYS)))
CURRENT_DAY=$(shell date +%d)
CURRENT_YEAR=2024
COOKIEFILE=cookies.txt

DAYS=$(wildcard rust/src/day*.rs)
COGABLE_FILES=$(shell find . -maxdepth 3 -type f -exec grep -q "\[\[\[cog" {} \; -print)

inputs: $(ALLINPUTS)
	@echo $^

help: ## Display this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.git/hooks/pre-commit: $(PRE_COMMIT_PATH) .pre-commit-config.yaml .git
	pre-commit install

init: .direnv .git/hooks/pre-commit $(UV_PATH) requirements.dev.txt ## Initalise a enviroment

clean: ## Remove all build files

$(COOKIEFILE):
	@echo "Please visit https://adventofcode.com, login and copy the session cookie to $@"
	@echo "format: session=xxxxxxxxxxxxxxxxx"
	@false

inputs/day_%.txt: $(COOKIEFILE)
	curl -H 'User-Agent: Makefile - curl : bengosney@googlemail.com' --cookie "$(shell cat $^)" -s -L -o $@ https://adventofcode.com/$(CURRENT_YEAR)/day/$(shell echo "$@" | egrep -o "[0-9]+" | sed 's/^0*//')/input

rust/src/day%.rs: ## Create a new rust file
	touch $@

today: inputs/day_$(CURRENT_DAY).txt rust/src/day$(CURRENT_DAY).rs rust/inputs ## Setup current day
	$(MAKE) cog

$(COGABLE_FILES): $(DAYS)
	uvx --from cogapp cog -cr $@
	@touch $@

cog: $(COGABLE_FILES) ## Run cog on all files
	
$(COG_PATH): .direnv
	@python -m uv pip install cogapp
	@touch $@

.direnv: .envrc
	@python -m pip install uv
	@touch $@ $^

rust/target/release/aoc2024: rust/src/main.rs $(DAYS)
	cd rust && cargo build --release

build: rust/target/release/aoc2024