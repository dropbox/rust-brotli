package brotli

import (
	"bytes"
	"fmt"
	"io"
	"io/ioutil"
	"testing"
)

var options = CompressionOptions{
	NumThreads: 1,
	Quality:    7,
	Catable:    true,
	Appendable: true,
	Magic:      true,
}

func TestCompressWriter(t *testing.T) {
	helpTestCompressWriter(t, false)
}
func TestMultiCompressWriter(t *testing.T) {
	helpTestCompressWriter(t, true)
}
func helpTestCompressWriter(t *testing.T, multi bool) {
	data := testData()
	outBuffer := bytes.NewBuffer(nil)
	var options = CompressionOptions{
		NumThreads: 1,
		Quality:    4,
		Catable:    true,
		Appendable: true,
		Magic:      true,
	}
	var writer io.WriteCloser
	if multi {
		writer = NewMultiCompressionWriter(
			outBuffer,
			options,
		)
	} else {
		writer = NewCompressionWriter(
			outBuffer,
			options,
		)
	}
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
	version, size, err := BrotliParseHeader(outBuffer.Bytes())
	if err != nil {
		panic(err)
	}
	if version != byte(BrotliEncoderVersion()&0xff) {
		panic(version)
	}
	if size != uint64(len(data)) {
		panic(size)
	}
}
func TestCompressRoundtrip(t *testing.T) {
	helpTestCompressRoundtrip(t, false)
}
func TestMultiCompressRoundtrip(t *testing.T) {
	helpTestCompressRoundtrip(t, true)
}
func helpTestCompressRoundtrip(t *testing.T, useMultiWriter bool) {
	tmp := testData()
	data := tmp[:len(tmp)-17]
	outBuffer := bytes.NewBuffer(nil)
	var options = CompressionOptions{
		NumThreads: 1,
		Quality:    9,
		Catable:    true,
		Appendable: true,
		Magic:      true,
	}
	var writer io.WriteCloser
	if useMultiWriter {
		writer = NewMultiCompressionWriter(
			NewDecompressionWriter(
				outBuffer,
			),
			options,
		)
	} else {
		writer = NewCompressionWriter(
			NewDecompressionWriter(
				outBuffer,
			),
			options,
		)
	}
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

func TestCompressFlushRoundtrip(t *testing.T) {
	tmp := testData()
	data := tmp[:len(tmp)-17]
	outBuffer := bytes.NewBuffer(nil)
	var options = CompressionOptions{
		NumThreads: 1,
		Quality:    9,
		Catable:    true,
		Appendable: true,
		Magic:      true,
	}
	writer := NewCompressionWriter(
		NewDecompressionWriter(
			outBuffer,
		),
		options,
	)
	hasFlushed := false
	delta := 15
	for dataIndex := 0; dataIndex < len(data); dataIndex += delta {
		end := dataIndex + delta
		if end > len(data) {
			end = len(data)
		}
		_, err := writer.Write(data[dataIndex:end])
		if err != nil {
			panic(err)
		}
		if dataIndex%255 == 0 {
			err = writer.Flush()
			hasFlushed = true
			if err != nil {
				panic(err)
			}
		}
	}
	if !hasFlushed {
		panic("test didn't trigger flush")
	}
	err := writer.Close()
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

type CounterWriteCloser struct {
	downstream io.WriteCloser
	count      int
}

func (mself *CounterWriteCloser) Write(data []byte) (int, error) {
	mself.count += len(data)
	return mself.downstream.Write(data)
}
func (mself *CounterWriteCloser) Close() error {
	return mself.downstream.Close()
}
func TestCompressAutoFlushRoundtrip(t *testing.T) {
	tmp := testData()
	data := tmp[:len(tmp)-17]
	outBuffer := bytes.NewBuffer(nil)
	var options = CompressionOptions{
		NumThreads:          1,
		Quality:             9,
		Catable:             true,
		Appendable:          true,
		Magic:               true,
		MaxBytesBeforeFlush: 255,
	}
	counterWriter := &CounterWriteCloser{NewDecompressionWriter(
		outBuffer,
	), 0}
	writer := NewCompressionWriter(
		counterWriter,
		options,
	)
	delta := 15
	lastDownstreamWriteCount := -1
	for dataIndex := 0; dataIndex < len(data); dataIndex += delta {
		end := dataIndex + delta
		if end > len(data) {
			end = len(data)
		}
		_, err := writer.Write(data[dataIndex:end])
		if err != nil {
			panic(err)
		}
		if dataIndex%255 == 0 {
			if lastDownstreamWriteCount == counterWriter.count {
				panic("Did not successfully flush between last write and 255 bytes")
			}
			lastDownstreamWriteCount = counterWriter.count
		}
	}
	err := writer.Close()
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

func TestCompressRoundtripMulti(*testing.T) {
	tmp := testData()
	data := tmp[:len(tmp)-17]
	outBuffer := bytes.NewBuffer(nil)
	var options = CompressionOptions{
		NumThreads: 16,
		Quality:    9,
		Catable:    true,
		Appendable: true,
		Magic:      true,
	}
	writer := NewMultiCompressionWriter(
		NewDecompressionWriter(
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
	var options = CompressionOptions{
		NumThreads: 1,
		Quality:    4,
		Catable:    true,
		Appendable: true,
		Magic:      true,
	}
	writer := NewMultiCompressionWriter(
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
	decompressorWriter := NewDecompressionWriter(
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
	decompressorWriter = NewDecompressionWriter(
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
	decompressorReader := NewDecompressionReader(corruptBuffer)
	_, err = ioutil.ReadAll(decompressorReader)
	if err == nil {
		panic("ExpectedError")
	}
	decompressorReader = NewDecompressionReader(compressedBuffer)
	_, err = ioutil.ReadAll(decompressorReader)
	if err != nil {
		panic(err)
	}
}
func TestCompressRoundtripZero(*testing.T) {
	var data []byte
	outBuffer := bytes.NewBuffer(nil)
	var options = CompressionOptions{
		NumThreads: 1,
		Quality:    9,
		Catable:    true,
		Appendable: true,
		Magic:      true,
	}
	compressedForm := bytes.NewBuffer(nil)
	writer := NewMultiCompressionWriter(
		io.MultiWriter(compressedForm, NewDecompressionWriter(
			outBuffer,
		),
		),
		options,
	)
	err := writer.Close()
	if err != nil {
		panic(err)
	}
	if len(compressedForm.Bytes()) == 0 {
		panic("Zero output buffer")
	}
	if !bytes.Equal(outBuffer.Bytes(), data[:]) {
		panic(fmt.Sprintf("Bytes not equal %d, %d", len(outBuffer.Bytes()), len(data)))
	}
}
func TestCompressReader(t *testing.T) {
	helpTestCompressReader(t, false)
}

func TestMultiCompressReader(t *testing.T) {
	helpTestCompressReader(t, true)
}

func helpTestCompressReader(t *testing.T, multi bool) {
	data := testData()
	inBuffer := bytes.NewBuffer(data[:])
	outBuffer := bytes.NewBuffer(nil)
	var options = CompressionOptions{
		NumThreads: 1,
		Quality:    4,
		Appendable: true,
		Magic:      true,
	}
	var reader io.ReadCloser
	if multi {
		reader = NewMultiCompressionReader(
			inBuffer,
			options,
		)
	} else {
		reader = NewCompressionReader(
			inBuffer,
			options,
		)
	}
	_, err := io.Copy(outBuffer, reader)
	if err != nil {
		panic(err)
	}
	if len(outBuffer.Bytes()) == 0 {
		panic("Zero output buffer")
	}
	if len(outBuffer.Bytes()) > 826011 {
		panic(fmt.Sprintf("Buffer too large: %d", len(outBuffer.Bytes())))
	}
	if multi && len(outBuffer.Bytes()) > 800000 {
		panic(fmt.Sprintf("Buffer too large for full size knowledge: %d", len(outBuffer.Bytes())))
	}
	version, size, err := BrotliParseHeader(outBuffer.Bytes())
	if err != nil {
		panic(err)
	}
	if version != byte(BrotliEncoderVersion()&0xff) {
		panic(version)
	}
	if multi && size != uint64(len(data)) {
		panic(size)
	}
}

func TestCompressReaderClose(t *testing.T) {
	helpTestCompressReaderClose(t, false)
}

func TestMultiCompressReaderClose(t *testing.T) {
	helpTestCompressReaderClose(t, true)
}

func helpTestCompressReaderClose(t *testing.T, multi bool) {
	data := testData()
	inBuffer := bytes.NewBuffer(data[:])
	outBuffer := bytes.NewBuffer(nil)
	var options = CompressionOptions{
		NumThreads: 1,
		Quality:    2,
		Catable:    true,
		Appendable: true,
		Magic:      true,
	}
	var reader io.ReadCloser
	if multi {
		reader = NewMultiCompressionReader(
			inBuffer,
			options,
		)
	} else {
		reader = NewCompressionReader(inBuffer, options)
	}
	_, err := io.Copy(outBuffer, reader)
	if err != nil {
		panic(err)
	}
	if len(outBuffer.Bytes()) == 0 {
		panic("Zero output buffer")
	}
	if len(outBuffer.Bytes()) > 1850280 {
		panic(fmt.Sprintf("Buffer too large: %d", len(outBuffer.Bytes())))
	}
	err = reader.Close()
	if err != nil {
		panic(err)
	}
	version, size, err := BrotliParseHeader(outBuffer.Bytes())
	if err != nil {
		panic(err)
	}
	if version != byte(BrotliEncoderVersion()&0xff) {
		panic(version)
	}
	if multi && size != uint64(len(data)) {
		panic(size)
	}
}

func TestCompressReaderEarlyClose(*testing.T) {
	data := testData()
	inBuffer := bytes.NewBuffer(data[:])
	var options = CompressionOptions{
		NumThreads: 1,
		Quality:    2,
		Catable:    true,
		Appendable: true,
		Magic:      true,
	}
	reader := NewMultiCompressionReader(
		inBuffer,
		options,
	)
	var smallBuf [1024]byte
	count, err := reader.Read(smallBuf[:])
	if err != nil {
		panic(err)
	}
	if count != len(smallBuf) {
		panic("Underflow for test data: too few bytes of test data")
	}
	err = reader.Close()
	if err != nil {
		panic(err)
	}
}

func TestCompressReaderRoundtrip(t *testing.T) {
	helpTestCompressReaderRoundtrip(t, false)
}
func TestMultiCompressReaderRoundtrip(t *testing.T) {
	helpTestCompressReaderRoundtrip(t, true)
}
func helpTestCompressReaderRoundtrip(t *testing.T, multi bool) {
	data := testData()
	inBuffer := bytes.NewBuffer(data[:])
	outBuffer := bytes.NewBuffer(nil)
	var options = CompressionOptions{
		NumThreads: 1,
		Quality:    4,
		Catable:    true,
		Appendable: true,
		Magic:      true,
	}
	var reader io.ReadCloser
	if multi {
		reader = NewDecompressionReader(
			NewMultiCompressionReader(
				inBuffer,
				options,
			),
		)
	} else {
		reader = NewDecompressionReader(
			NewCompressionReader(
				inBuffer,
				options,
			),
		)
	}
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

func TestDecompressReaderEarlyClose(*testing.T) {
	data := testData()
	inBuffer := bytes.NewBuffer(data[:])
	var options = CompressionOptions{
		NumThreads: 1,
		Quality:    4,
		Catable:    true,
		Appendable: true,
		Magic:      true,
	}
	reader := NewDecompressionReader(
		NewMultiCompressionReader(
			inBuffer,
			options,
		),
	)
	var smallBuffer [1027]byte
	count, err := reader.Read(smallBuffer[:])
	if err != nil {
		panic(err)
	}
	if count < 1024 {
		panic("Too small a test buffer")
	}
	err = reader.Close()
	if err != nil {
		panic(err)
	}
	if !bytes.Equal(smallBuffer[:], data[:len(smallBuffer)]) {
		panic(fmt.Sprintf("Bytes not equal %x, %x", smallBuffer[:], data[:len(smallBuffer)]))
	}
}
func TestCompressReaderRoundtripZero(t *testing.T) {
	helpTestCompressReaderRoundtripZero(t, false)
}
func TestMultiCompressReaderRoundtripZero(t *testing.T) {
	helpTestCompressReaderRoundtripZero(t, true)
}
func helpTestCompressReaderRoundtripZero(t *testing.T, multi bool) {
	var data []byte
	inBuffer := bytes.NewBuffer(data[:])
	outBuffer := bytes.NewBuffer(nil)
	var options = CompressionOptions{
		NumThreads: 1,
		Quality:    11,
		Catable:    true,
		Appendable: true,
		Magic:      true,
	}
	compressedForm := bytes.NewBuffer(nil)
	reader := NewDecompressionReader(
		io.TeeReader(
			NewMultiCompressionReader(
				inBuffer,
				options,
			),
			compressedForm),
	)
	_, err := io.Copy(outBuffer, reader)
	if err != nil {
		panic(err)
	}
	if len(compressedForm.Bytes()) == 0 {
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
	_, err = io.Copy(midBufferA, NewMultiCompressionReader(
		inBufferAa,
		options,
	))
	if err != nil {
		panic(err)
	}
	midBufferB := bytes.NewBuffer(nil)
	_, err = io.Copy(midBufferB, NewMultiCompressionReader(
		inBufferBa,
		options,
	))
	if err != nil {
		panic(err)
	}
	midBufferC := bytes.NewBuffer(nil)
	_, err = io.Copy(midBufferC, NewMultiCompressionReader(
		inBufferCa,
		options,
	))
	if err != nil {
		panic(err)
	}
	midBufferD := bytes.NewBuffer(nil)
	_, err = io.Copy(midBufferD, NewMultiCompressionReader(
		inBufferDa,
		options,
	))
	if err != nil {
		panic(err)
	}
	final, err := BroccoliConcat([][]byte{midBufferA.Bytes(), midBufferB.Bytes(), midBufferC.Bytes(), midBufferD.Bytes()}...)
	if err != nil {
		panic(err)
	}
	finalBuffer := bytes.NewBuffer(final)
	rtBuffer := bytes.NewBuffer(nil)
	_, err = io.Copy(rtBuffer, NewDecompressionReader(finalBuffer))
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
	var options = CompressionOptions{
		NumThreads: 1,
		Quality:    4,
		Catable:    true,
		Appendable: true,
		Magic:      true,
	}

	reader := NewDecompressionReader(
		NewBroccoliConcatReader(
			NewMultiCompressionReader(
				inBufferA,
				options,
			),
			NewMultiCompressionReader(
				inBufferB,
				options,
			),
			NewMultiCompressionReader(
				inBufferC,
				options,
			),
			NewMultiCompressionReader(
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
	if BrotliEncoderVersion() == 0 {
		panic(fmt.Sprintf("Bad version %d\n", BrotliEncoderVersion()))
	}
	if BrotliDecoderVersion() == 0 {
		panic(fmt.Sprintf("Bad version %d\n", BrotliDecoderVersion()))
	}
}
