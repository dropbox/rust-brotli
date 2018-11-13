package main

/*
#cgo CFLAGS: -I..
#cgo LDFLAGS: target/debug/libbrotli_ffi.a -lm -ldl
#include <brotli/encode.h>
#include <brotli/decode.h>
#include <brotli/multiencode.h>
*/
import "C"
import (
	"errors"
	"io"
	"os"
	"unsafe"
)

type CompressionOptions struct {
	NumThreads int
	Quality    float32
}

type MultiCompressionReader struct {
	options  CompressionOptions
	buffer   []byte
	output   []byte
	upstream io.Reader
}

func NewMultiCompressionReader(
	upstream io.Reader,
	options CompressionOptions,
) *MultiCompressionReader {
	return &MultiCompressionReader{options: options, upstream: upstream}
}

func (mself *MultiCompressionReader) Read(data []byte) (int, error) {
	if mself.upstream != nil {
		for mself.upstream != nil {
			size, err := mself.upstream.Read(data)
			mself.buffer = append(mself.buffer, data[:size]...)
			if err == io.EOF {
				mself.upstream = nil
				break
			}
		}
		var bufferAddr *C.uint8_t
		if len(mself.buffer) != 0 {
			bufferAddr = (*C.uint8_t)(&mself.buffer[0])
		}
		mself.output = make([]byte, int(C.BrotliEncoderMaxCompressedSizeMulti(
			C.size_t(len(mself.buffer)), C.size_t(mself.options.NumThreads))))
		parameters, values, numParams := makeCompressionOptionsStreams(
			mself.options,
		)
		outputLen := C.size_t(len(mself.output))
		ret := C.BrotliEncoderCompressMulti(
			numParams,
			parameters,
			values,
			C.size_t(len(mself.buffer)),
			bufferAddr,
			&outputLen,
			(*C.uint8_t)(&mself.output[0]),
			C.size_t(mself.options.NumThreads),
			nil, nil, nil,
		)
		if ret == 0 {
			return 0, errors.New("Compression failed")
		}
	}
	toCopy := len(data)
	if len(mself.output) < toCopy {
		toCopy = len(mself.output)
	}
	copy(data[:toCopy], mself.output[:toCopy])
	mself.output = mself.output[toCopy:]
	if len(mself.output) == 0 {
		mself.output = nil
		return toCopy, io.EOF
	}
	return toCopy, nil
}

func makeCompressionOptionsStreams(options CompressionOptions,
) (*C.BrotliEncoderParameter, *C.uint32_t, C.size_t) {
	qualityParam := C.BrotliEncoderParameter(C.BROTLI_PARAM_QUALITY)
	qualityParams := []C.BrotliEncoderParameter{qualityParam}
	values := []C.uint32_t{C.uint32_t(options.Quality)}
	return &qualityParams[0], &values[0], 1
}

type MultiCompressionWriter struct {
	options    CompressionOptions
	buffer     []byte
	downstream io.Writer
}

func NewMultiCompressionWriter(
	downstream io.Writer,
	options CompressionOptions,
) *MultiCompressionWriter {
	return &MultiCompressionWriter{options: options, downstream: downstream}
}

func (mself *MultiCompressionWriter) Write(data []byte) (int, error) {
	mself.buffer = append(mself.buffer, data...)
	return len(data), nil
}

func (mself *MultiCompressionWriter) Close() error {
	output := make([]byte, int(C.BrotliEncoderMaxCompressedSizeMulti(
		C.size_t(len(mself.buffer)), C.size_t(mself.options.NumThreads))))
	var bufferAddr *C.uint8_t
	if len(mself.buffer) != 0 {
		bufferAddr = (*C.uint8_t)(&mself.buffer[0])
	}
	parameters, values, numParams := makeCompressionOptionsStreams(
		mself.options,
	)
	outputLen := C.size_t(len(output))
	ret := C.BrotliEncoderCompressMulti(
		numParams,
		parameters,
		values,
		C.size_t(len(mself.buffer)),
		bufferAddr,
		&outputLen,
		(*C.uint8_t)(&output[0]),
		C.size_t(mself.options.NumThreads),
		nil, nil, nil)
	if ret == 0 {
		return errors.New("Compression failed")
	}
	_, err := mself.downstream.Write(output[:outputLen])
	if err != nil {
		return err
	}
	if writeCloser, ok := mself.downstream.(io.WriteCloser); ok {
		return writeCloser.Close() // SHOULD close downstream?
	}
	return nil
}

const BufferSize = 16384

func NewMultiDecompressionReader(upstream io.Reader) io.Reader {
	panic("UNIMPL")
}

type DecompressionWriter struct {
	downstream io.Writer
	state      *C.BrotliDecoderState
	buffer     [BufferSize]byte
	done       bool
}

func NewMultiDecompressionWriter(downstream io.Writer) *DecompressionWriter {
	return &DecompressionWriter{
		downstream: downstream,
		state:      C.BrotliDecoderCreateInstance(nil, nil, nil),
	}
}

func (mself *DecompressionWriter) Write(data []byte) (int, error) {
	if mself.state == nil {
		return 0, errors.New("Write on closed DecompressionWriter")
	}
	avail_in := C.size_t(len(data))
	for avail_in != 0 {
		if mself.done {
			return 0, io.ErrShortWrite
		}
		last_start := C.size_t(len(data)) - avail_in
		next_in := &data[last_start]
		avail_out := C.size_t(len(mself.buffer))
		next_out := &mself.buffer[0]
		ret := C.BrotliDecoderDecompressStream(
			mself.state,
			&avail_in,
			(**C.uint8_t)(unsafe.Pointer(&next_in)),
			&avail_out,
			(**C.uint8_t)(unsafe.Pointer(&next_out)),
			nil,
		)
		to_copy := C.size_t(len(mself.buffer)) - avail_out
		if to_copy != 0 {
			mself.downstream.Write(mself.buffer[:to_copy])
		}
		if ret == C.BROTLI_DECODER_RESULT_ERROR {
			return 0, errors.New(C.GoString(C.BrotliDecoderGetErrorString(mself.state)))
		}
		if ret == C.BROTLI_DECODER_RESULT_SUCCESS {
			mself.done = true
			if avail_in != 0 {
				return 0, io.ErrShortWrite
			}
			break
		}
	}
	return len(data), nil
}

func (mself *DecompressionWriter) Close() error {
	if mself.state != nil {
		C.BrotliDecoderDestroyInstance(mself.state)
		mself.state = nil
	}
	if writeCloser, ok := mself.downstream.(io.WriteCloser); ok {
		err := writeCloser.Close()
		if err != nil {
			return err
		}
	}
	if !mself.done {
		return io.ErrUnexpectedEOF
	}
	return nil
}

func main() {
	decompress := false
	useWriter := false
	for _, arg := range os.Args {
		if arg == "-w" {
			useWriter = true
		}
		if arg == "-d" {
			decompress = true
		}
	}
	if useWriter {
		var writer io.Writer
		if decompress {
			writer = NewMultiDecompressionWriter(
				os.Stdout,
			)
		} else {
			writer = NewMultiCompressionWriter(
				os.Stdout,
				CompressionOptions{
					NumThreads: 8,
					Quality:    11,
				},
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
			reader = NewMultiDecompressionReader(
				os.Stdin,
			)
		} else {
			reader = NewMultiCompressionReader(
				os.Stdin,
				CompressionOptions{
					NumThreads: 8,
					Quality:    9.5,
				},
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
