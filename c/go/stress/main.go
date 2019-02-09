package main

import (
//	"io"
//	"io/ioutil"
//	"os"
	"flag"

	"github.com/dropbox/rust-brotli/c/go/brotli"
)

var useWriter = flag.Bool("use_writer", false, "should use writer for brotli")
var parallelism = flag.Int("workers", 16, "number of workers to run with")
var threads = flag.Int("threads", 2, "number of threads to compress the file")
var iters = flag.Int("iter", 1024, "number of times to run the compression/decompression cycle")
var size = flag.Int("size", 4096 * 1024, "Size of the test data")
var quality = flag.Int("quality", 5, "brotli quality level")

func main() {
	flag.Parse()
	options := brotli.CompressionOptions{
		NumThreads: *threads,
		Quality:    5,
		Catable:    true,
		Appendable: true,
		Magic:      true,
	}
	_ = useWriter
    _ = options
}
