test:
	cargo test --locked  --no-fail-fast -- --test-threads 3

run:
	docker-compose up --build

down:
	docker-compose down