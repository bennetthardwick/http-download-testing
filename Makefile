build:
	go build go/main.go
	(cd rust && cargo build --release)

bench_go:
	./main | pv > /dev/null

bench_rust:
	./rust/target/release/rust | pv > /dev/null

bench_curl:
	curl -s --http1.1 ${TEST_URL} | pv > /dev/null
