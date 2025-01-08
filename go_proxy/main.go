package main

import (
	"crypto/tls"
	"io"
	"log"
	"net/http"
	"os"

	"github.com/gorilla/mux"
)

func main() {
	url, ok := os.LookupEnv("TEST_URL")

	l := log.New(os.Stderr, "", 0)

	if !ok {
		l.Println("Export TEST_URL with file to download")
		os.Exit(1)
	}

	l.Printf("Starting Go server on localhost:2000 to serve %s", url)

	client := http.Client{
		Transport: &http.Transport{
			// disable http2
			TLSNextProto: make(map[string]func(authority string, c *tls.Conn) http.RoundTripper),
		},
	}

	r := mux.NewRouter()
	r.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		req, err := http.NewRequest("GET", url, nil)

		if err != nil {
			l.Println("failed to create request")
			os.Exit(1)
		}

		auth, ok := os.LookupEnv("TEST_AUTH")

		if ok {
			l.Println("Using TEST_AUTH header")
			req.Header.Add("Authorization", "Bearer "+auth)
		}

		res, err := client.Do(req)

		if err != nil {
			l.Println("Request returned error")
			os.Exit(1)
		}

		io.Copy(w, res.Body)
	})

	srv := http.Server{
		Handler:      r,
		Addr:         "127.0.0.1:2000",
	}

	l.Fatal(srv.ListenAndServe())
}
