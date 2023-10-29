rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run etl

release:
	cargo build --release

all: format lint test run


etl: 
	cargo run etl

create: 
	cargo run create "INSERT INTO world_billionaires (rank, final_worth, category, person_name, age, country, city, source, self_made, gender, last_name, first_name) VALUES (1, 211000, 'Technology', 'Haliunaa Munkhuu', 28, 'United States', 'Durham', 'Noogle', TRUE, 'F', 'Haliunaa', 'Munkhuu');"

read: 
	cargo run read "SELECT * FROM world_billionaires WHERE category = 'Technology' AND age > 90;"

update:
	cargo run update "UPDATE world_billionaires SET final_worth = 300000 WHERE person_name = 'Mark Zuckerberg';"

delete:
	cargo run delete "DELETE FROM world_billionaires WHERE person_name = 'Elon Musk';"
