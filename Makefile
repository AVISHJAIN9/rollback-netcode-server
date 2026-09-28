.PHONY: all build test clean docker-up docker-down

all: build

build:
	cd backend && cargo build
	cd frontend && npm install && npm run build

test:
	cd backend && cargo test
	python3 aiml/eval/evaluate_cheat_detection.py

bench:
	cd backend && cargo bench

docker-up:
	docker-compose -f infra/docker-compose.yml up -d

docker-down:
	docker-compose -f infra/docker-compose.yml down

clean:
	cd backend && cargo clean
	rm -rf frontend/.next frontend/node_modules
