package main

import (
	"crypto/tls"
	"io"
	"log"
	"net/http"
	"os"
)

func main() {
	url, ok := os.LookupEnv("TEST_URL")

	l := log.New(os.Stderr, "", 0)

	if !ok {
		l.Println("Export TEST_URL with file to download")
		os.Exit(1)
	}

	l.Printf("Fetching %s with Go http", url)

	client := http.Client{
		Transport: &http.Transport{
			// disable http2
			TLSNextProto: make(map[string]func(authority string, c *tls.Conn) http.RoundTripper),
		},
	}

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

	io.Copy(os.Stdout, res.Body)
}
