
.PHONY: windows
windows: ## Build clip binaries for Windows.
	@cross build --release --target=x86_64-pc-windows-gnu

.PHONY: linux
linux: ## Build clip binaries for linux.
	@cross build --release --target=x86_64-unknown-linux-gnu
