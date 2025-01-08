build:
	(cd go && go build main.go)
	(cd go_proxy && go build main.go)
	cargo build --release

bench_go:
	./go/main | pv -a > /dev/null

bench_reqwest:
	./target/release/reqwest_bench | pv -a > /dev/null

bench_isahc:
	./target/release/isahc_bench | pv -a > /dev/null

bench_isahc_stream_reader:
	./target/release/isahc_stream_reader_bench | pv -a > /dev/null

bench_ureq:
	./target/release/ureq_bench | pv -a > /dev/null

bench_curl:
	curl -s --http1.1 ${TEST_URL} | pv -a > /dev/null

bench:
	$(MAKE) bench_curl
	$(MAKE) bench_go
	$(MAKE) bench_isahc
	$(MAKE) bench_isahc_stream_reader
	$(MAKE) bench_reqwest
	$(MAKE) bench_ureq
