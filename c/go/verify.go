package main

import (
	"bytes"
	"fmt"

	"github.com/dropbox/rust-brotli/c/go/brotli"
	"io"
	insecure_random "math/rand"
	"os"
	"path/filepath"
)

func makeOptions() brotli.CompressionOptions {
	ret := brotli.CompressionOptions{
		NumThreads: insecure_random.Intn(15) + 1,
		Quality:    float32(insecure_random.Intn(9) + 3),
		Catable:    true,
		Appendable: true,
		Magic:      true,
	}
	if ret.Quality > 9 {
		ret.Quality = 9.5
	}
	return ret
}
func verifyReader(path string) {
	f, err := os.Open(path)
	if err != nil {
		return
	}
	defer func() { _ = f.Close() }()
	origFile := bytes.NewBuffer(nil)
	options := makeOptions()
	reader := brotli.NewMultiCompressionReader(
		io.TeeReader(f, origFile),
		options,
	)
	compressed := bytes.NewBuffer(nil)
	rtReader := brotli.NewDecompressionReader(io.TeeReader(reader, compressed))
	rt := bytes.NewBuffer(nil)
	io.Copy(rt, rtReader)
	if string(rt.Bytes()) != string(origFile.Bytes()) {
		fi, err := os.Create("/tmp/IN.orig")
		if err != nil {
			defer func() { _ = fi.Close() }()
			_, _ = fi.Write(origFile.Bytes())
		}
		fc, err := os.Create("/tmp/IN.br")
		if err != nil {
			defer func() { _ = fc.Close() }()
			_, _ = fc.Write(compressed.Bytes())
		}
		fr, err := os.Create("/tmp/IN.rt")
		if err != nil {
			defer func() { _ = fr.Close() }()
			_, _ = fr.Write(rt.Bytes())
		}
		panic(fmt.Sprintf("%v Bytes mismatch %d != %d\n", options, len(rt.Bytes()), len(origFile.Bytes())))
	}
}

func verifyWriter(path string) {
	f, err := os.Open(path)
	if err != nil {
		return
	}
	options := makeOptions()
	defer func() { _ = f.Close() }()
	compressed := bytes.NewBuffer(nil)
	rt := bytes.NewBuffer(nil)
	origFile := bytes.NewBuffer(nil)
	dwriter := brotli.NewDecompressionWriter(rt)
	writer := brotli.NewMultiCompressionWriter(
		io.MultiWriter(compressed, dwriter),
		options,
	)
	_, err = io.Copy(io.MultiWriter(writer, origFile), f)
	if err != nil {
		panic(err)
	}
	err = writer.Close()
	if err != nil {
		panic(err)
	}
	err = dwriter.Close()
	if err != nil {
		panic(err)
	}
	if string(rt.Bytes()) != string(origFile.Bytes()) {
		fi, err := os.Create("/tmp/INW.orig")
		if err != nil {
			defer func() { _ = fi.Close() }()
			_, _ = fi.Write(origFile.Bytes())
		}
		fc, err := os.Create("/tmp/INW.br")
		if err != nil {
			defer func() { _ = fc.Close() }()
			_, _ = fc.Write(compressed.Bytes())
		}
		fr, err := os.Create("/tmp/INW.rt")
		if err != nil {
			defer func() { _ = fr.Close() }()
			_, _ = fr.Write(rt.Bytes())
		}
		panic(fmt.Sprintf("%v Bytes mismatch %d != %d\n", options, len(rt.Bytes()), len(origFile.Bytes())))
	} else {
		fmt.Fprintf(os.Stderr, "Processing %s %v/%v = %f\n",
			path, len(rt.Bytes()), len(compressed.Bytes()), float32(len(compressed.Bytes()))/float32(len(rt.Bytes())))
	}
}

func recursiveVerify(root string) {
	filepath.Walk(root, filepath.WalkFunc(func(path string, info os.FileInfo, err error) error {
		if info.Size() > 1 && !info.IsDir() {
			func() {
				f, err := os.Open(path)
				if err != nil {
					return
				}
				defer func() { _ = f.Close() }()
				var test [64]byte
				sz, err := f.Read(test[:])
				if sz == 0 || err != nil {
					return
				}
				fmt.Fprintf(os.Stderr, "Processing %v (%d)\n", path, info.Size())
				verifyReader(path)
				verifyWriter(path)
			}()
		}
		return nil
	}))
}
