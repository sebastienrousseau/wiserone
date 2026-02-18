# Wiserone Development Makefile
# Enforces same quality standards as CI pipeline

.PHONY: help check fmt fmt-check lint test coverage security docs bench clean all ci-local install-tools

# Default target
all: check fmt-check lint test coverage security docs bench

# Help target
help: ## Show this help message
	@echo "🛠️  Wiserone Development Commands"
	@echo ""
	@echo "Quality Gates (CI Enforcement):"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'
	@echo ""
	@echo "⚡ Use 'make ci-local' to run all CI checks locally"

# Environment setup
install-tools: ## Install required development tools
	@echo "🔧 Installing development tools..."
	@rustup component add rustfmt clippy llvm-tools-preview
	@cargo install cargo-tarpaulin cargo-audit cargo-deny --locked
	@echo "✅ Development tools installed"

# Formatting
fmt: ## Format code using rustfmt
	@echo "🎨 Formatting code..."
	@cargo fmt --all
	@echo "✅ Code formatted"

fmt-check: ## Check code formatting (CI enforcement)
	@echo "🔍 Checking code formatting..."
	@cargo fmt --all -- --check
	@echo "✅ Code formatting is correct"

# Type checking
check: ## Run type checking
	@echo "🔧 Type checking..."
	@cargo check --workspace --all-targets --all-features
	@echo "✅ Type checking passed"

# Linting with zero-warning policy
lint: ## Run clippy lints (zero-warning policy)
	@echo "🔍 Running clippy lints (zero-warning policy)..."
	@RUSTFLAGS="-Dwarnings" cargo clippy --workspace --all-targets --all-features --no-deps -- -D warnings
	@echo "✅ No linting issues found"

# Testing
test: ## Run test suite
	@echo "🧪 Running test suite..."
	@RUSTFLAGS="-Dwarnings" cargo test --workspace --all-features --verbose
	@echo "✅ All tests passed"

# Coverage with 100% threshold
coverage: ## Run tests with coverage (100% threshold)
	@echo "📊 Running coverage analysis (100% threshold)..."
	@cargo tarpaulin --workspace --all-features --out Html --output-dir coverage/ --fail-under 100 --verbose --timeout 300
	@echo "✅ Coverage requirement met (100%)"
	@echo "📄 Coverage report: coverage/tarpaulin-report.html"

# Security scanning
security: ## Run security audits (NO || true allowed)
	@echo "🛡️ Running security audits..."
	@echo "  🔍 Vulnerability scan..."
	@cargo audit
	@echo "  🏛️ License compliance..."
	@cargo deny check
	@echo "✅ Security audits passed"

# Documentation
docs: ## Generate and test documentation
	@echo "📚 Generating documentation..."
	@RUSTDOCFLAGS="-Dwarnings" cargo doc --workspace --all-features --no-deps --document-private-items
	@echo "  🧪 Testing doc examples..."
	@cargo test --doc --workspace --all-features
	@echo "✅ Documentation generated and tested"

# Benchmarks
bench: ## Run performance benchmarks
	@echo "📊 Running benchmarks..."
	@cargo bench --workspace --all-features
	@echo "✅ Benchmarks completed"

# Build release
build: ## Build release version
	@echo "🏗️ Building release..."
	@RUSTFLAGS="-Dwarnings" cargo build --release --workspace --all-features
	@echo "✅ Release build completed"

# Clean artifacts
clean: ## Clean build artifacts
	@echo "🧹 Cleaning build artifacts..."
	@cargo clean
	@rm -rf coverage/
	@echo "✅ Clean completed"

# Full CI simulation locally
ci-local: install-tools clean ## Run full CI pipeline locally
	@echo "🚀 Running full CI pipeline locally..."
	@echo ""
	@echo "=========================================="
	@echo "🔍 Stage 1: Pre-flight Checks"
	@echo "=========================================="
	@$(MAKE) fmt-check
	@$(MAKE) lint
	@$(MAKE) check
	@echo ""
	@echo "=========================================="
	@echo "🧪 Stage 2: Test Suite"
	@echo "=========================================="
	@$(MAKE) test
	@echo ""
	@echo "=========================================="
	@echo "📊 Stage 3: Coverage Analysis"
	@echo "=========================================="
	@$(MAKE) coverage
	@echo ""
	@echo "=========================================="
	@echo "🛡️ Stage 4: Security Audit"
	@echo "=========================================="
	@$(MAKE) security
	@echo ""
	@echo "=========================================="
	@echo "📚 Stage 5: Documentation"
	@echo "=========================================="
	@$(MAKE) docs
	@echo ""
	@echo "=========================================="
	@echo "📊 Stage 6: Benchmarks"
	@echo "=========================================="
	@$(MAKE) bench
	@echo ""
	@echo "🎉 All CI checks passed locally!"
	@echo "✅ Ready for pull request"

# Development workflow targets
dev-setup: install-tools ## Setup development environment
	@echo "🏗️ Setting up development environment..."
	@rustup toolchain install stable nightly
	@rustup default stable
	@$(MAKE) check
	@echo "✅ Development environment ready"

quick-check: fmt-check lint check ## Quick pre-commit checks
	@echo "⚡ Quick pre-commit checks completed"

# Nightly tools testing
test-nightly: ## Test with nightly compiler
	@echo "🌙 Testing with nightly compiler..."
	@cargo +nightly test --workspace --all-features
	@echo "✅ Nightly tests passed"

# Utility targets
version: ## Show current version
	@echo "📦 Current version: $(shell grep '^version' Cargo.toml | head -n1 | cut -d'"' -f2)"

deps-update: ## Update dependencies (dry run)
	@echo "🔄 Checking for dependency updates..."
	@cargo update --dry-run

# Container testing (if Docker available)
test-docker: ## Test in clean container environment
	@if command -v docker >/dev/null 2>&1; then \
		echo "🐳 Testing in Docker container..."; \
		docker run --rm -v $(PWD):/workspace -w /workspace rust:latest bash -c "cargo test --workspace --all-features"; \
		echo "✅ Docker tests passed"; \
	else \
		echo "⚠️ Docker not available, skipping container test"; \
	fi

# IDE integration helpers
vscode-setup: ## Setup VS Code settings for project
	@mkdir -p .vscode
	@echo '{"rust-analyzer.check.command": "clippy", "rust-analyzer.check.allTargets": false}' > .vscode/settings.json
	@echo "✅ VS Code settings configured"

# Performance profiling
profile: ## Run performance profiling
	@echo "⚡ Running performance profiling..."
	@cargo build --release --workspace
	@echo "Use tools like 'perf', 'valgrind', or 'cargo flamegraph' for detailed profiling"

# Show project stats
stats: ## Show project statistics
	@echo "📊 Project Statistics:"
	@echo "  Lines of code: $(shell find src tests -name '*.rs' -exec cat {} \; | wc -l)"
	@echo "  Rust files: $(shell find src tests -name '*.rs' | wc -l)"
	@echo "  Dependencies: $(shell grep -c '=' Cargo.toml | head -1)"
	@echo "  Test files: $(shell find tests -name '*.rs' | wc -l)"