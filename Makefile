# Default docker-compose command
COMPOSE = docker-compose exec rust_backend

# Run the application
run:
	$(COMPOSE) cargo run
	@echo ""

# Run migrations
migrate:
	$(COMPOSE) cargo run migrate
	@echo ""

# Seed all tables
seed-all:
	$(COMPOSE) cargo run seed --table all
	@echo ""

# Seed a specific table
seed:
	$(COMPOSE) cargo run seed --table $(TABLE)
	@echo ""

# Generate entities using sea-orm-cli
generate-entities:
	docker-compose run --rm sea-orm-cli
	@echo ""