
help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

clean: ## Clean the project using cargo
	cargo clean

build: ## Build the project using cargo
	cargo build

lint: ## Lint the project using cargo
	@rustup component add clippy 2> /dev/null
	cargo clippy

fmt: ## Format the project using cargo
	@rustup component add rustfmt 2> /dev/null
	cargo fmt

test: ## Test the project using cargo
	cargo test

release: ## Build the project using cargo in release mode
	cargo build --release

create: ## Specific qeury to insert new info into BasellDB
	cargo run "INSERT INTO BaseballDB (Name,Team, Position) VALUES ('Mike Keohane', 'WAS', 'Baller');"

read: ## Specific qeury to read info from BaseballDB
	cargo run "SELECT * FROM BaseballDB LIMIT 15;"

update: ## Specific qeury to update info in BaseballDB
	cargo run "UPDATE BaseballDB SET Team = 'Washington' WHERE Team = 'WAS';"

delete: ## Specific qeury to delete info in BaseballDB
	cargo run "DELETE FROM BaseballDB WHERE Name = 'Mike Keohane';"