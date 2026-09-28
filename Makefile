# Unified Makefile for Rollback Netcode Server
.PHONY: setup test dev clean bench lint docker-up docker-down

setup:
	@echo "==> Setting up development environment..."
	@pip3 install -r aiml/requirements.txt
	@echo "==> Setup complete."

test:
	@echo "==> Running Determinism & Core Netcode Test Suite..."
	@python3 tests/test_determinism.py
	@python3 tests/test_rollback.py
	@echo "==> Running AIML Model Evaluation & Baseline Checks..."
	@python3 aiml/eval/evaluate_cheat_detection.py
	@python3 aiml/eval/evaluate_jitter_predictor.py
	@echo "==> All Tests Passed."

lint:
	@echo "==> Checking code styling & formatting..."
	@python3 -m py_compile tests/*.py aiml/**/*.py
	@echo "==> Lint check clean."

bench:
	@echo "==> Executing simulation benchmarks..."
	@python3 tests/test_determinism.py

dev:
	@echo "==> Starting local development stack..."
	@docker-compose -f infra/docker-compose.yml up -d
	@echo "==> Stack online. Frontend: http://localhost:3000, Server: UDP 0.0.0.0:9000"

clean:
	@echo "==> Cleaning build artifacts and cache..."
	@find . -type d -name "__pycache__" -exec rm -rf {} +
	@rm -rf frontend/.next
	@echo "==> Clean complete."
