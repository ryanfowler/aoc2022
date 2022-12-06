setup-day%:
	@./setup.sh $*

build-day%: setup-day%
	@cargo build --release --bin day$*

run-day%: build-day%
	@./target/release/day$*

docker-build-day%: setup-day%
	@DOCKER_BUILDKIT=1 docker build -t day$* --build-arg DAY_NUM=$* .

docker-run-day%: docker-build-day%
	@docker run -it --rm day$*
