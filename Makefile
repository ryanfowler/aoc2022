setup-day%:
	@./setup.sh $*

build-day%: setup-day%
	@cargo build --release --bin day$*

run-day%: build-day%
	@./target/release/day$*
