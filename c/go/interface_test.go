package main

import (
	//"io/ioutil"
	//"os"
	"bytes"
	"fmt"
	"io"
	"testing"
	"github.com/dropbox/rust-brotli/c/go/brotli"
)

var options = brotli.CompressionOptions{
	NumThreads: 1,
	Quality:    7,
	Catable:    true,
	Appendable: true,
	Magic:      true,
}

func TestCompressWriter(*testing.T) {
	data := testData()
	outBuffer := bytes.NewBuffer(nil)
	var options = brotli.CompressionOptions{
		NumThreads: 1,
		Quality:    4,
		Catable:    true,
		Appendable: true,
		Magic:      true,
	}
	writer := brotli.NewMultiCompressionWriter(
		outBuffer,
		options,
	)
	_, err := writer.Write(data[:])
	if err != nil {
		panic(err)
	}
	err = writer.Close()
	if err != nil {
		panic(err)
	}
	if len(outBuffer.Bytes()) == 0 {
		panic("Zero output buffer")
	}
	if len(outBuffer.Bytes()) > 800000 {
		panic(fmt.Sprintf("Buffer too large: %d", len(outBuffer.Bytes())))
	}
}
func TestCompressRoundtrip(*testing.T) {
	tmp := testData()
	data := tmp[:len(tmp)-17]
	outBuffer := bytes.NewBuffer(nil)
	var options = brotli.CompressionOptions{
		NumThreads: 1,
		Quality:    9,
		Catable:    true,
		Appendable: true,
		Magic:      true,
	}
	writer := brotli.NewMultiCompressionWriter(
		brotli.NewDecompressionWriter(
			outBuffer,
		),
		options,
	)
	_, err := writer.Write(data[:])
	if err != nil {
		panic(err)
	}
	err = writer.Close()
	if err != nil {
		panic(err)
	}
	if len(outBuffer.Bytes()) == 0 {
		panic("Zero output buffer")
	}
	if !bytes.Equal(outBuffer.Bytes(), data[:]) {
		panic(fmt.Sprintf("Bytes not equal %d, %d", len(outBuffer.Bytes()), len(data)))
	}
}
func TestCompressReader(*testing.T) {
	data := testData()
	inBuffer := bytes.NewBuffer(data[:])
	outBuffer := bytes.NewBuffer(nil)
	var options = brotli.CompressionOptions{
		NumThreads: 1,
		Quality:    4,
		Catable:    true,
		Appendable: true,
		Magic:      true,
	}
	reader := brotli.NewMultiCompressionReader(
		inBuffer,
		options,
	)
	_, err := io.Copy(outBuffer, reader)
	if err != nil {
		panic(err)
	}
	if len(outBuffer.Bytes()) == 0 {
		panic("Zero output buffer")
	}
	if len(outBuffer.Bytes()) > 800000 {
		panic(fmt.Sprintf("Buffer too large: %d", len(outBuffer.Bytes())))
	}
}
func TestCompressReaderRoundtrip(*testing.T) {
	data := testData()
	inBuffer := bytes.NewBuffer(data[:])
	outBuffer := bytes.NewBuffer(nil)
	var options = brotli.CompressionOptions{
		NumThreads: 1,
		Quality:    4,
		Catable:    true,
		Appendable: true,
		Magic:      true,
	}
	reader := brotli.NewDecompressionReader(
		brotli.NewMultiCompressionReader(
			inBuffer,
			options,
		),
	)
	_, err := io.Copy(outBuffer, reader)
	if err != nil {
		panic(err)
	}
	if len(outBuffer.Bytes()) == 0 {
		panic("Zero output buffer")
	}
	if !bytes.Equal(outBuffer.Bytes(), data[:]) {
		panic(fmt.Sprintf("Bytes not equal %d, %d", len(outBuffer.Bytes()), len(data)))
	}
}

func TestConcatFlatFunction(*testing.T) {
	data := testData()
	inBufferAa := bytes.NewBuffer(data[:len(data)/5])
	inBufferBa := bytes.NewBuffer(data[len(data)/5 : 2*(len(data)/5)])
	inBufferCa := bytes.NewBuffer(data[2*(len(data)/5) : 3*(len(data)/5)])
	inBufferDa := bytes.NewBuffer(data[3*(len(data)/5):])
	midBufferA := bytes.NewBuffer(nil)
	var err error
	_, err = io.Copy(midBufferA, brotli.NewMultiCompressionReader(
		inBufferAa,
		options,
	))
	if err != nil {
		panic(err)
	}
	midBufferB := bytes.NewBuffer(nil)
	_, err = io.Copy(midBufferB, brotli.NewMultiCompressionReader(
		inBufferBa,
		options,
	))
	if err != nil {
		panic(err)
	}
	midBufferC := bytes.NewBuffer(nil)
	_, err = io.Copy(midBufferC, brotli.NewMultiCompressionReader(
		inBufferCa,
		options,
	))
	if err != nil {
		panic(err)
	}
	midBufferD := bytes.NewBuffer(nil)
	_, err = io.Copy(midBufferD, brotli.NewMultiCompressionReader(
		inBufferDa,
		options,
	))
	if err != nil {
		panic(err)
	}
	final, err := brotli.BroccoliConcat([][]byte{midBufferA.Bytes(), midBufferB.Bytes(), midBufferC.Bytes(), midBufferD.Bytes()}...)
	if err != nil {
		panic(err)
	}
	finalBuffer := bytes.NewBuffer(final)
	rtBuffer := bytes.NewBuffer(nil)
	_, err = io.Copy(rtBuffer, brotli.NewDecompressionReader(finalBuffer))
	if err != nil {
		panic(err)
	}
	if !bytes.Equal(rtBuffer.Bytes(), data[:]) {
		panic(fmt.Sprintf("Bytes not equal %d, %d", len(rtBuffer.Bytes()), len(data)))
	}
}

func TestConcatReaderRoundtrip(*testing.T) {
	data := testData()
	inBufferA := bytes.NewBuffer(data[:len(data)/5-1])
	inBufferB := bytes.NewBuffer(data[len(data)/5-1 : 2+2*(len(data)/5)])
	inBufferC := bytes.NewBuffer(data[2+2*(len(data)/5) : 3*(len(data)/5)])
	inBufferD := bytes.NewBuffer(data[3*(len(data)/5):])
	outBuffer := bytes.NewBuffer(nil)
	var options = brotli.CompressionOptions{
		NumThreads: 1,
		Quality:    4,
		Catable:    true,
		Appendable: true,
		Magic:      true,
	}

	reader := brotli.NewDecompressionReader(
		brotli.NewBroccoliConcatReader(
			brotli.NewMultiCompressionReader(
				inBufferA,
				options,
			),
			brotli.NewMultiCompressionReader(
				inBufferB,
				options,
			),
			brotli.NewMultiCompressionReader(
				inBufferC,
				options,
			),
			brotli.NewMultiCompressionReader(
				inBufferD,
				options,
			),
		))
	_, err := io.Copy(outBuffer, reader)
	if err != nil {
		panic(err)
	}
	if len(outBuffer.Bytes()) == 0 {
		panic("Zero output buffer")
	}
	if !bytes.Equal(outBuffer.Bytes(), data[:]) {
		panic(fmt.Sprintf("Bytes not equal %d, %d", len(outBuffer.Bytes()), len(data)))
	}
}

/*
	useWriter := false—
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
			final, err := BroccoliConcat(buffers...)
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
				if err != —nil {
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
			writer = brotli.NewMultiCompressionWriter(
				os.Stdout,
				options,
			)
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
			reader = brotli.NewMultiCompressionReader(
				os.Stdin,
				options,
			)
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
*/
