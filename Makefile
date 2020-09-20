test:
	cargo test --locked  --no-fail-fast -- --test-threads 3

run:
	docker-compose up --build

quick-run:
	docker-compose up

down:
	docker-compose down