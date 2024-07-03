PROJECT_ROOT := $(shell pwd)

.PHONY: server

generate-model:
	@cargo install sea-orm-cli
	@sea-orm-cli generate entity -v --with-serde=both -o src/entities \
		--max-connections=10 --ignore-tables=migration --seaography \
		-u "mysql://root:99158194@127.0.0.1:3306/resource_website"

dependency-up:
	@docker-compose up -d

dependency-down:
	@docker-compose down

server:
	@cargo run 
