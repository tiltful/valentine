install-wasm-pack:
	@echo "Installing wasm-pack..."
	cargo install wasm-pack

wasm:
	@echo "Building wasm..."
	wasm-pack build --target web

serve:
	@echo "Serving..."
	python3 -m http.server

build:
	@echo "Building..."
	cargo build

release:
	@echo "Building release..."
	cargo build --release

changelog:
	@echo "Generating changelog..."
	git cliff --config detailed > CHANGELOG.md
	@echo "Done."