.PHONY: all setup-day% build-day% run-all run-day% docker-build-day% docker-run-day%

all: run-all

setup-day%:
	@./setup.sh $*

build-day%: setup-day%
	@cargo --quiet build --release --bin day$*

run-all:
	@echo "Compiling all available days...\n"
	@ls -d ./src/* | grep -o '..$$' | xargs -P 16 -I % sh -c 'make -s setup-day%'
	@ls -d ./src/* | grep -o '..$$' | xargs -I {} echo "--bin day{}" | xargs -n 32 sh -c 'cargo build -q --release'
	@ls -d ./src/* | grep -o '..$$' | time -p xargs -I % sh -c './target/release/day% && echo'

run-day%: build-day%
	@./target/release/day$*

docker-build-day%: setup-day%
	@DOCKER_BUILDKIT=1 docker build -t day$* --build-arg DAY_NUM=$* .

docker-run-day%: docker-build-day%
	@docker run -it --rm day$*
