all:
	# Documentation
	cargo doc --workspace --document-private-items

	# Checks
	cargo clippy -- -F clippy::missing_docs_in_private_items

	# Tests
	cargo test -- --nocapture

	# Benchmarking
	cargo bench
