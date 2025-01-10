build:
	(cd go && go build main.go)
	(cd go_proxy && go build main.go)
	cargo build --release

bench_go:
	./go/main | pv > /dev/null

bench_reqwest:
	./target/release/reqwest_bench | pv > /dev/null

bench_reqwest_spawn:
	./target/release/reqwest_spawn_bench | pv > /dev/null

bench_reqwest_rustls:
	./target/release/reqwest_rustls_bench | pv > /dev/null

bench_isahc:
	./target/release/isahc_bench | pv > /dev/null

bench_isahc_spawn:
	./target/release/isahc_spawn_bench | pv > /dev/null

bench_isahc_blocking:
	./target/release/isahc_blocking_bench | pv > /dev/null

bench_isahc_stream_reader:
	./target/release/isahc_stream_reader_bench | pv > /dev/null

bench_ureq:
	./target/release/ureq_bench | pv > /dev/null

bench_curl:
	curl -s --http1.1 '${TEST_URL}' | pv > /dev/null

bench:
	@echo "Running curl"
	$(MAKE) bench_curl
	@echo "Running go"
	$(MAKE) bench_go
	@echo "Running isahc"
	$(MAKE) bench_isahc
	@echo "Running isahc_stream_reader"
	$(MAKE) bench_isahc_stream_reader
	@echo "Running reqwest"
	$(MAKE) bench_reqwest
	@echo "Running reqwest_rustls"
	$(MAKE) bench_reqwest_rustls
	@echo "Running ureq"
	$(MAKE) bench_ureq

build_push_ubuntu:
	docker build -f ./Dockerfile.ubuntu -t "bennetthardwick/http-request-testing:ubuntu-24.04" .
	docker push "bennetthardwick/http-request-testing:ubuntu-24.04"

build_push_alpine:
	docker build -f ./Dockerfile.alpine -t "bennetthardwick/http-request-testing:alpine-3.21" .
	docker push "bennetthardwick/http-request-testing:alpine-3.21"

build_push:
	$(MAKE) build_push_alpine
	$(MAKE) build_push_ubuntu
