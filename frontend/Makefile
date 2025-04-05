# frontend/Makefile

# =======================================
# TailYew Docs Frontend Makefile
# Located at: frontend/Makefile
# Purpose: Build and serve the static Yew + Tailwind documentation site
# =======================================

# ---------------------------------------
# Configuration
# ---------------------------------------
ENV_FILE ?= .env.development     # Default environment file (can be overridden)
STATIC_DIR ?= ./static           # Directory for static assets
BUILD_DIR ?= ./static/pkg        # Directory for WebAssembly build artifacts
PORT ?= 3030                     # Default port for the development server

# =======================================
# Build Targets
# =======================================

# Build Tailwind CSS into the static directory
build-tailwind:
	@echo "Building Tailwind CSS..."
	npx tailwindcss  -i main.css \
		-c tailwind.config.js \
		-o ./static/styles.css --minify

# Build WebAssembly package using wasm-pack
build-wasm:
	@echo "Building WebAssembly package..."
	wasm-pack build \
		--target web \
		--out-dir $(BUILD_DIR) \
		--out-name frontend

# Build both CSS and WebAssembly
build: clean
	@echo "Starting build process..."
	@echo "Using ENV_FILE=$(ENV_FILE)"
	make build-tailwind
	make build-wasm
	@echo "Build completed."

# =======================================
# Run and Serve Targets
# =======================================

# Start the Warp server for development
start:
	@echo "Starting development server on http://localhost:$(PORT)..."
	cd static && serve -s -l $(PORT) --no-clipboard

# Run the development server with Warp and build WASM
run: build start

# =======================================
# Cleanup Targets
# =======================================

# Clean up build artifacts
clean:
	@echo "Cleaning up build artifacts..."
	rm -rf $(BUILD_DIR) ./static/styles.css
	@echo "Cleanup completed."
