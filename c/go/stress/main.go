package main

import (
	"syscall"
	//	"io/ioutil"
	"time"
	"bytes"
	"compress/zlib"
	"crypto/md5"
	"io"
	"flag"
	"sync"

	"github.com/dropbox/rust-brotli/c/go/brotli"
)

var useWriter = flag.Bool("use_writer", false, "should use writer for brotli")
var parallelism = flag.Int("workers", 16, "number of workers to run with")
var threads = flag.Int("threads", 2, "number of threads to compress the file")
var iters = flag.Int("iter", 1024 * 1024, "number of times to run the compression/decompression cycle")
var sizeHint = flag.Uint64("size", 4096 * 1024, "Size of the test data")
var quality = flag.Float64("quality", 5, "brotli quality level")
var useZlib = flag.Int("zlib", 0, "1 for zlib 2 for both zlib and brotli")
var nongraceful = flag.Bool("hardexit", false, "use syscall.ExitGroup to end the program")
var anongoroutines = flag.Bool("anon", false, "use a separate anonymous goroutine for each invocation")
var timeout = flag.Duration("timeout", 0, "timeout until process exits with code 1")
func main() {
	flag.Parse()
	if *timeout != 0 {
		go func() {
			time.Sleep(*timeout)
			syscall.Exit(1)
		}()
	}
	options := brotli.CompressionOptions{
		NumThreads: *threads,
		Quality:    float32(*quality),
		Catable:    true,
		Appendable: true,
		Magic:      true,
		SizeHint:   uint32(*sizeHint),
	}
	file := testData(int(options.SizeHint))
	if *anongoroutines {
		var wg sync.WaitGroup
		wg.Add(*parallelism)
		for par :=0; par < *parallelism; par += 1 {
			go func() {
				for iter := 0; iter < *iters; iter += 1 {
					_ = stresst(options, file, *useZlib % 2 == 0)
					if *useZlib == 2 {
						_ = stresst(options, file, false)
					}
				}
				if *nongraceful {
					syscall.Exit(0)
				}
				wg.Done()
			}()
		}
		wg.Wait()
	} else {
		for iter := 0; iter < *iters; iter += 1 {
			var wg sync.WaitGroup
			wg.Add(*parallelism)
			for par :=0; par < *parallelism; par += 1 {
				go func() {
					_ = stresst(options, file, *useZlib % 2 == 0)
					if *useZlib == 2 {
						_ = stresst(options, file, false)
					}
					wg.Done()
				}()
			}
			wg.Wait()
		}
		if *nongraceful {
			syscall.Exit(0)
		}
	}
}

func stresst(
	options brotli.CompressionOptions,
	file []byte,
	useBrotli bool,
) (
	md5out [md5.Size]byte,
) {
	var output bytes.Buffer
	input := bytes.NewBuffer(file)
	var compressionWriter io.WriteCloser
	var err error
	if useBrotli {
		compressionWriter = brotli.NewMultiCompressionWriter(
			&output, options)
	} else {
		compressionWriter = zlib.NewWriter(&output)
	}
	if err != nil {
		panic(err)
	}
	_, err = io.Copy(compressionWriter, input)
	if err != nil {
		panic(err)
	}
	err = compressionWriter.Close()
	if err != nil {
		panic(err)
	}
	md5out = md5.Sum(output.Bytes())
	compressed := bytes.NewBuffer(output.Bytes())
	var compressionReader io.ReadCloser
	if useBrotli {
		compressionReader = brotli.NewDecompressionReader(compressed)
	} else {
		compressionReader, err = zlib.NewReader(compressed)
	}
	if err != nil {
		panic(err)
	}
	var rt bytes.Buffer
	_, err = io.Copy(&rt, compressionReader)
	if err != nil {
		panic(err)
	}
	err = compressionReader.Close()
	if err != nil {
		panic(err)
	}
	if !bytes.Equal(rt.Bytes(), file) {
		panic ("Files differ " + string(rt.Bytes()))
	}
	return
}
