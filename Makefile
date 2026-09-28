# Unified Makefile for Rollback Netcode Server
.PHONY: setup test dev clean bench lint docker-up docker-down

setup:
	@echo "==> Setting up development environment..."
	@pip3 install -r aiml/requirements.txt
	@echo "==> Setup complete."

test:
	@echo "==> [Gate G1 & G2] 1,000,000 Step Determinism & Golden State Verification..."
	@python3 tests/test_determinism.py
	@python3 tests/test_golden_determinism.py
	@python3 tests/test_rollback.py
	@echo "==> [Gate G3] Multiplayer Session & Reconnect Verification..."
	@python3 tests/test_multiplayer_session.py
	@echo "==> [Gate G4] Security, Rate Limiting & Fuzzing Verification..."
	@python3 tests/test_security_fuzzing.py
	@echo "==> [Gate G5] Network Chaos & Packet Loss Redundancy Verification..."
	@python3 tests/test_chaos_network.py
	@echo "==> [Gate G6] AIML Models, Safety Guardrails & MLOps Verification..."
	@python3 aiml/eval/evaluate_all_models.py
	@echo "==> [Gate G7] E2E Multiplayer Lifecycle Verification..."
	@python3 tests/test_e2e_flow.py
	@echo "==> [Gate G8] Client SDK Smoke Verification..."
	@python3 tests/test_sdk_smoke.py
	@echo ""
	@echo "=================================================================="
	@echo " ALL RELEASE GATES G1 THROUGH G8 VERIFIED 100% GREEN."
	@echo "=================================================================="

lint:
	@echo "==> Checking code syntax..."
	@python3 -m py_compile tests/*.py aiml/**/*.py
	@echo "==> Syntax clean."

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
