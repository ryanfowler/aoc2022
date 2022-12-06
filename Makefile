build-day%:
	@cargo build --release --bin day$*

run-day%: build-day%
	@./target/release/day$*
