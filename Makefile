build:
	go build go/main.go
	(cd reqwest && cargo build --release)
	(cd isahc && cargo build --release)
	(cd ureq && cargo build --release)
	(cd server && cargo build --release)

bench_go:
	./main | pv > /dev/null

bench_reqwest:
	./reqwest/target/release/bench | pv > /dev/null

bench_isahc:
	./isahc/target/release/bench | pv > /dev/null

bench_ureq:
	./ureq/target/release/bench | pv > /dev/null

bench_curl:
	curl -s --http1.1 ${TEST_URL} | pv > /dev/null
