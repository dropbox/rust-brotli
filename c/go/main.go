package main

import (
	"github.com/dropbox/rust-brotli/c/go/brotli"
	"io"
	"io/ioutil"
	"os"
)

func main() {
	decompress := false
	options := brotli.CompressionOptions{
		NumThreads: 1,
		Quality:    9.5,
		Catable:    true,
		Appendable: true,
		Magic:      false,
	}
	useWriter := false
	var toCat []string
	for index, arg := range os.Args {
		if index == 0 {
			continue
		}
		if arg == "-w" {
			useWriter = true
		}
		if arg == "-d" {
			decompress = true
		}
		if arg == "-dirtree" {
			recursiveVerify(os.Args[index+1])
			return
		}
		if arg == "-cat" {
			toCat = append(toCat, os.Args[index+1:]...)
			break
		}
	}
	if toCat != nil {
		if useWriter {
			buffers := make([][]byte, len(toCat))
			for index, fn := range toCat {
				var err error
				buffers[index], err = ioutil.ReadFile(fn)
				if err != nil {
					panic(err)
				}
			}
			final, err := brotli.BroccoliConcat(buffers...)
			if err != nil {
				panic(err)
			}
			_, err = os.Stdout.Write(final)
			if err != nil {
				panic(err)
			}
		} else {
			files := make([]io.Reader, len(toCat))
			for index, fn := range toCat {
				var err error
				files[index], err = os.Open(fn)
				if err != nil {
					panic(err)
				}
			}
			_, err := io.Copy(os.Stdout, brotli.NewBroccoliConcatReader(files...))
			if err != nil {
				panic(err)
			}
			for _, file := range files {
				if readCloser, ok := file.(io.ReadCloser); ok {
					_ = readCloser.Close()
				}
			}
		}
		return
	} else if useWriter {
		var writer io.Writer
		if decompress {
			writer = brotli.NewDecompressionWriter(
				os.Stdout,
			)
		} else {
			if options.NumThreads == 1 {
				writer = brotli.NewCompressionWriter(
					os.Stdout,
					options,
				)
			} else {
				writer = brotli.NewMultiCompressionWriter(
					os.Stdout,
					options,
				)
			}
		}
		for {
			var buffer [65536]byte
			count, err := os.Stdin.Read(buffer[:])
			if err == io.EOF {
				break
			}
			if err != nil {
				panic(err)
			}
			_, err = writer.Write(buffer[:count])
			if err != nil {
				panic(err)
			}
		}
		if writeCloser, ok := writer.(io.WriteCloser); ok {
			err := writeCloser.Close()
			if err != nil {
				panic(err)
			}
		}
	} else {
		var reader io.Reader
		if decompress {
			reader = brotli.NewDecompressionReader(
				os.Stdin,
			)
		} else {
			if options.NumThreads == 1 {
				reader = brotli.NewCompressionReader(
					os.Stdin,
					options,
				)
			} else {
				reader = brotli.NewMultiCompressionReader(
					os.Stdin,
					options,
				)
			}
		}
		for {
			var buffer [65536]byte
			size, err := reader.Read(buffer[:])
			_, werr := os.Stdout.Write(buffer[:size])
			if werr != nil {
				panic(werr)
			}
			if err == io.EOF {
				return
			}
			if err != nil {
				panic(err)
			}
		}
	}
}
