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

	client := http.Client{
		Transport: &http.Transport{
			// disable http2
			TLSNextProto: make(map[string]func(authority string, c *tls.Conn) http.RoundTripper),
		},
	}

	res, err := client.Get(url)

	if err != nil {
		l.Println("Request returned error")
		os.Exit(1)
	}

	io.Copy(os.Stdout, res.Body)
}
