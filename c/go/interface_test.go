package main

import (
	"bytes"
	"fmt"
	"io"
	"io/ioutil"
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

func TestRejectCorruptBuffers(*testing.T) {
	tmp := testData()
	data := tmp[:len(tmp)-17]
	outBuffer := bytes.NewBuffer(nil)
	compressedBuffer := bytes.NewBuffer(nil)
	var options = brotli.CompressionOptions{
		NumThreads: 1,
		Quality:    4,
		Catable:    true,
		Appendable: true,
		Magic:      true,
	}
	writer := brotli.NewMultiCompressionWriter(
		compressedBuffer,
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
	decompressorWriter := brotli.NewDecompressionWriter(
		outBuffer,
	)
	// early EOF
	_, err = decompressorWriter.Write(compressedBuffer.Bytes()[:len(compressedBuffer.Bytes())-1])
	if err != nil {
		panic(err)
	}
	err = decompressorWriter.Close()
	if err == nil {
		panic("Expected error")
	}
	decompressorWriter = brotli.NewDecompressionWriter(
		outBuffer,
	)
	_, err = decompressorWriter.Write(compressedBuffer.Bytes()[:len(compressedBuffer.Bytes())/2])
	if err != nil {
		panic(err)
	}
	// missed a byte
	_, err = decompressorWriter.Write(compressedBuffer.Bytes()[len(compressedBuffer.Bytes())/2+1:])
	if err == nil {
		panic("ExpectedError")
	}
	_ = decompressorWriter.Close()
	corruptBuffer := bytes.NewBuffer(compressedBuffer.Bytes()[:len(compressedBuffer.Bytes())-1])
	decompressorReader := brotli.NewDecompressionReader(corruptBuffer)
	_, err = ioutil.ReadAll(decompressorReader)
	if err == nil {
		panic("ExpectedError")
	}
	decompressorReader = brotli.NewDecompressionReader(compressedBuffer)
	_, err = ioutil.ReadAll(decompressorReader)
	if err != nil {
		panic(err)
	}
}
func TestCompressRoundtripZero(*testing.T) {
	var data []byte
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
    _, err := writer.Write(data)
    
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

func TestCompressReaderRoundtripZero(*testing.T) {
    var data []byte
	inBuffer := bytes.NewBuffer(data[:])
	outBuffer := bytes.NewBuffer(nil)
	var options = brotli.CompressionOptions{
		NumThreads: 1,
		Quality:    11,
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

func TestVersions(*testing.T) {
	if brotli.BrotliEncoderVersion() == 0 {
		panic(fmt.Sprintf("Bad version %d\n", brotli.BrotliEncoderVersion()))
	}
	if brotli.BrotliDecoderVersion() == 0 {
		panic(fmt.Sprintf("Bad version %d\n", brotli.BrotliDecoderVersion()))
	}
}
