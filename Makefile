PROJECT_ROOT := $(shell pwd)

.PHONY: server

dependency-up:
	@docker-compose up -d

dependency-down:
	@docker-compose down

server:
	@cargo run 