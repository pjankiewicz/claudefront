.PHONY: help backend frontend export-openapi generate-sdk sdk dev clean test

help:
	@echo "Strategy Game - Makefile Commands"
	@echo ""
	@echo "  make backend           - Run the Rust backend server"
	@echo "  make frontend          - Run the frontend dev server"
	@echo "  make export-openapi    - Export OpenAPI spec from backend"
	@echo "  make generate-sdk      - Generate TypeScript SDK (types, models, services)"
	@echo "  make sdk              - Export OpenAPI + Generate complete SDK"
	@echo "  make dev              - Run both backend and frontend"
	@echo "  make test             - Test frontend build (TypeScript + Vite)"
	@echo "  make clean            - Clean build artifacts"

backend:
	@echo "Checking for process on port 3000..."
	@lsof -ti:3000 | xargs kill -9 2>/dev/null || true
	@echo "Starting backend server..."
	cd backend && cargo run

frontend:
	cd frontend && npm install && npm run dev

export-openapi:
	@echo "Starting backend server..."
	@cd backend && cargo run > /dev/null 2>&1 & echo $$! > /tmp/strategy-game-backend.pid
	@echo "Waiting for server to start..."
	@sleep 8
	@echo "Downloading OpenAPI spec..."
	@curl -s http://localhost:3000/api-docs/openapi.json > backend/openapi.json
	@echo "Stopping backend server..."
	@kill `cat /tmp/strategy-game-backend.pid` 2>/dev/null || true
	@rm /tmp/strategy-game-backend.pid 2>/dev/null || true
	@echo "✅ OpenAPI spec exported to backend/openapi.json"

generate-sdk:
	@echo "Installing frontend dependencies..."
	@cd frontend && npm install --silent
	@echo "Cleaning old SDK..."
	@rm -rf frontend/src/api
	@echo "Generating TypeScript SDK (types, models, services) from OpenAPI spec..."
	@cd frontend && npx openapi-typescript-codegen --input ../backend/openapi.json --output src/api --client fetch --useOptions
	@echo "✅ TypeScript SDK generated in frontend/src/api/"
	@echo "   - Types, models, and services are ready to use"

sdk: export-openapi generate-sdk
	@echo "✅ SDK generation complete!"

dev:
	@echo "Cleaning up port 3000..."
	@lsof -ti:3000 | xargs kill -9 2>/dev/null || true
	@sleep 1
	@echo "Starting backend and frontend..."
	@make -j2 backend frontend

test:
	@echo "Testing frontend build..."
	@cd frontend && npm install --silent
	@cd frontend && npm run build
	@echo "✅ Frontend build test passed!"

clean:
	@echo "Cleaning build artifacts..."
	@cd backend && cargo clean
	@cd frontend && rm -rf node_modules dist
	@rm -f backend/openapi.json
	@echo "✅ Clean complete"
