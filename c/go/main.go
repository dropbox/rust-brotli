package main

/*
#cgo CFLAGS: -I..
#cgo LDFLAGS: -L../target/release -l brotli_ffi -lm -ldl
#include <brotli/encode.h>
#include <brotli/decode.h>
#include <brotli/broccoli.h>
#include <brotli/multiencode.h>
static BrotliDecoderResult BrDecompressStream(BrotliDecoderState* s,
                                            size_t *avail_in,
                                            const uint8_t* in,
                                            size_t *avail_out,
                                            uint8_t* out,
                                            size_t* total_bytes_written) {
  return BrotliDecoderDecompressStream(
      s, avail_in, &in, avail_out, &out, NULL);
}
static BroccoliResult BrConcatStream(BroccoliState *s,
                                     size_t * available_in,
                                     const uint8_t *input_buf,
                                     size_t *available_out,
                                     uint8_t *output_buf) {
    return BroccoliConcatStream(s, available_in, &input_buf, available_out, &output_buf);
}
static BroccoliResult BrConcatFinish(BroccoliState *s,
                                     size_t *available_out,
                                     uint8_t *output_buf) {
    return BroccoliConcatFinish(s, available_out, &output_buf);
}
*/
import "C"
import (
	"errors"
	"fmt"
	"io"
	"os"
	"unsafe"
)

type CompressionOptions struct {
	NumThreads                    int
	Quality                       float32
	Catable                       bool
	Appendable                    bool
	Magic                         bool
	Mode                          int
	LgWin                         byte
	LgBlock                       byte
	DisableLiteralContextModeling bool
	SizeHint                      uint32
	NumDirect                     uint32
	NumPostfix                    uint32
	LiteralByteScore              uint32
	AvoidDistancePrefixSearch     bool
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
		mself.output = mself.output[:int(outputLen)]
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
	qualityParams := []C.BrotliEncoderParameter{C.BROTLI_PARAM_QUALITY}
	values := []C.uint32_t{C.uint32_t(options.Quality)}
	if options.Quality > 9 && options.Quality < 10 {
		values = append(values, 1)
		qualityParams = append(qualityParams, C.BROTLI_PARAM_Q9_5)
		if options.Quality <= 9.5 {
			values[0] = 10 // q9.5
		} else {
			values[0] = 11 // q9.6
		}
	}
	if options.Catable {
		qualityParams = append(qualityParams, C.BROTLI_PARAM_CATABLE)
		values = append(values, 1)
	}
	if options.Appendable {
		qualityParams = append(qualityParams, C.BROTLI_PARAM_APPENDABLE)
		values = append(values, 1)
	}
	if options.Magic {
		qualityParams = append(qualityParams, C.BROTLI_PARAM_MAGIC_NUMBER)
		values = append(values, 1)
	}
	if options.Mode != 0 {
		qualityParams = append(qualityParams, C.BROTLI_PARAM_MODE)
		values = append(values, C.uint32_t(options.Mode))
	}
	if options.LgWin != 0 {
		qualityParams = append(qualityParams, C.BROTLI_PARAM_LGWIN)
		values = append(values, C.uint32_t(options.LgWin))
	}

	if options.LgBlock != 0 {
		qualityParams = append(qualityParams, C.BROTLI_PARAM_LGBLOCK)
		values = append(values, C.uint32_t(options.LgBlock))
	}
	if options.DisableLiteralContextModeling {
		qualityParams = append(qualityParams, C.BROTLI_PARAM_DISABLE_LITERAL_CONTEXT_MODELING)
		values = append(values, 1)
	}
	if options.SizeHint != 0 {
		qualityParams = append(qualityParams, C.BROTLI_PARAM_SIZE_HINT)
		values = append(values, C.uint32_t(options.SizeHint))
	}
	if options.NumDirect != 0 {
		qualityParams = append(qualityParams, C.BROTLI_PARAM_NDIRECT)
		values = append(values, C.uint32_t(options.NumDirect))
	}
	if options.NumPostfix != 0 {
		qualityParams = append(qualityParams, C.BROTLI_PARAM_NPOSTFIX)
		values = append(values, C.uint32_t(options.NumPostfix))
	}
	if options.LiteralByteScore != 0 {
		qualityParams = append(qualityParams, C.BROTLI_PARAM_LITERAL_BYTE_SCORE)
		values = append(values, C.uint32_t(options.LiteralByteScore))
	}
	if options.AvoidDistancePrefixSearch {
		qualityParams = append(qualityParams, C.BROTLI_PARAM_AVOID_DISTANCE_PREFIX_SEARCH)
		values = append(values, 1)
	}

	return &qualityParams[0], &values[0], C.size_t(len(qualityParams))
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

type DecompressionReader struct {
	upstream   io.Reader
	state      *C.BrotliDecoderState
	buffer     [BufferSize]byte
	validStart int
	validEnd   int
	eof        bool
}

func NewDecompressionReader(upstream io.Reader) *DecompressionReader {
	return &DecompressionReader{
		upstream: upstream,
		state:    C.BrotliDecoderCreateInstance(nil, nil, nil),
	}
}

func (mself *DecompressionReader) populateBuffer() error {
	for mself.validStart == mself.validEnd && !mself.eof {
		var err error
		mself.validEnd, err = mself.upstream.Read(mself.buffer[:])
		mself.validStart = 0
		if err != nil {
			if err == io.EOF {
				mself.eof = true
				break
			} else {
				return err
			}
		}
	}
	return nil
}
func (mself *DecompressionReader) Read(data []byte) (int, error) {
	if mself.state == nil {
		return 0, io.EOF
	}
	if len(data) == 0 {
		return 0, nil
	}
	for {
		err := mself.populateBuffer()
		if err != nil {
			return 0, err
		}
		avail_in := C.size_t(mself.validEnd - mself.validStart)
		next_in := &mself.buffer[0]
		if avail_in != 0 {
			next_in = &mself.buffer[mself.validStart]
		}
		avail_out := C.size_t(len(data))
		next_out := &data[0]
		ret := C.BrDecompressStream(
			mself.state,
			&avail_in,
			(*C.uint8_t)(next_in),
			&avail_out,
			(*C.uint8_t)(next_out),
			nil,
		)
		mself.validStart = mself.validEnd - int(avail_in)
		if ret == C.BROTLI_DECODER_RESULT_ERROR {
			err := errors.New(C.GoString(C.BrotliDecoderGetErrorString(mself.state)))
			C.BrotliDecoderDestroyInstance(mself.state)
			mself.state = nil
			return 0, err
		}
		if ret == C.BROTLI_DECODER_RESULT_SUCCESS {
			C.BrotliDecoderDestroyInstance(mself.state)
			mself.state = nil
			return len(data) - int(avail_out), io.EOF
		}
		if ret == C.BROTLI_DECODER_NEEDS_MORE_INPUT && mself.validStart == mself.validEnd && mself.eof {
			return len(data) - int(avail_out), io.ErrUnexpectedEOF
		}
		if int(avail_out) != len(data) {
			return len(data) - int(avail_out), nil
		}
	}
}

type DecompressionWriter struct {
	downstream io.Writer
	state      *C.BrotliDecoderState
	buffer     [BufferSize]byte
	done       bool
}

func NewDecompressionWriter(downstream io.Writer) *DecompressionWriter {
	return &DecompressionWriter{
		downstream: downstream,
		state:      C.BrotliDecoderCreateInstance(nil, nil, nil),
	}
}

func (mself *DecompressionWriter) Write(data []byte) (int, error) {
	if mself.state == nil {
		return 0, errors.New("Write on closed DecompressionWriter")
	}
	if len(data) == 0 {
		return 0, nil
	}
	avail_in := C.size_t(len(data))
	for {
		if mself.done {
			return 0, io.ErrShortWrite
		}
		last_start := C.size_t(len(data)) - avail_in
		next_in := (*C.uint8_t)(unsafe.Pointer(&mself.buffer[0])) // only if size == 0, in which case it won't be read
		if avail_in != 0 {
			next_in = (*C.uint8_t)(unsafe.Pointer(&data[last_start]))
		}
		avail_out := C.size_t(len(mself.buffer))
		next_out := &mself.buffer[0]
		ret := C.BrDecompressStream(
			mself.state,
			&avail_in,
			(*C.uint8_t)(next_in),

			&avail_out,
			(*C.uint8_t)(next_out),
			nil,
		)
		to_copy := C.size_t(len(mself.buffer)) - avail_out
		if to_copy != 0 {
			_, err := mself.downstream.Write(mself.buffer[:to_copy])
			if err != nil {
				return int(last_start), err
			}
		}
		if ret == C.BROTLI_DECODER_RESULT_ERROR {
			err := errors.New(C.GoString(C.BrotliDecoderGetErrorString(mself.state)))
			C.BrotliDecoderDestroyInstance(mself.state)
			mself.state = nil

			return 0, err
		}
		if avail_in == 0 && ret == C.BROTLI_DECODER_NEEDS_MORE_INPUT {
			break
		}
		if ret == C.BROTLI_DECODER_RESULT_SUCCESS {
			mself.done = true
			if avail_in != 0 {
				return len(data) - int(avail_in), io.ErrShortWrite
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

type BroccoliConcatReader struct {
	upstreams  []io.Reader
	state      C.BroccoliState
	buffer     [BufferSize]byte
	validStart int
	validEnd   int
}

func NewBroccoliConcatReaderWithWindowSize(windowSize byte, upstreams ...io.Reader) *BroccoliConcatReader {
	ret := &BroccoliConcatReader{
		upstreams: upstreams,
		state:     C.BroccoliCreateInstanceWithWindowSize(C.uint8_t(windowSize)),
	}
	if len(ret.upstreams) != 0 {
		C.BroccoliNewBrotliFile(&ret.state)
	}
	return ret
}

func NewBroccoliConcatReader(upstreams ...io.Reader) *BroccoliConcatReader {
	ret := &BroccoliConcatReader{
		upstreams: upstreams,
		state:     C.BroccoliCreateInstance(),
	}
	if len(ret.upstreams) != 0 {
		C.BroccoliNewBrotliFile(&ret.state)
	}
	return ret
}

func (mself *BroccoliConcatReader) populateBuffer() error {
	for mself.validStart == mself.validEnd && len(mself.upstreams) != 0 {
		var err error
		mself.validEnd, err = mself.upstreams[0].Read(mself.buffer[:])
		mself.validStart = 0
		if err != nil {
			if err == io.EOF {
				mself.upstreams = mself.upstreams[1:]
				if len(mself.upstreams) != 0 {
					C.BroccoliNewBrotliFile(&mself.state)
				} else {
					break
				}
			} else {
				return err
			}
		}
	}
	return nil
}
func (mself *BroccoliConcatReader) Read(data []byte) (int, error) {
	if len(data) == 0 {
		return 0, nil
	}
	for {
		err := mself.populateBuffer()
		if err != nil {
			return 0, err
		}
		avail_out := C.size_t(len(data))
		avail_in := C.size_t(mself.validEnd - mself.validStart)
		var ret C.BroccoliResult
		if avail_in != 0 {
			ret = C.BrConcatStream(
				&mself.state,
				&avail_in,
				(*C.uint8_t)(&mself.buffer[mself.validStart]),
				&avail_out,
				(*C.uint8_t)(&data[0]),
			)
		} else {
			if len(mself.upstreams) != 0 {
				return 0, errors.New("Invariant Violation: avail upstreams but no bytes to read from")
			}
			ret = C.BrConcatFinish(
				&mself.state,
				&avail_out,
				(*C.uint8_t)(&data[0]),
			)
		}
		mself.validStart = mself.validEnd - int(avail_in)
		if ret != C.BroccoliSuccess && ret != C.BroccoliNeedsMoreInput && ret != C.BroccoliNeedsMoreOutput {
			err := fmt.Errorf("Broccoli Error Code: %v", ret)
			C.BroccoliDestroyInstance(mself.state)
			return 0, err
		}
		if ret == C.BroccoliSuccess {
			C.BroccoliDestroyInstance(mself.state)
			return len(data) - int(avail_out), io.EOF
		}
		if ret == C.BroccoliNeedsMoreInput && mself.validStart == mself.validEnd && len(mself.upstreams) == 0 {
			return len(data) - int(avail_out), io.ErrUnexpectedEOF
		}
		if int(avail_out) != len(data) {
			return len(data) - int(avail_out), nil
		}
	}
}

func broccoliConcat(state C.BroccoliState, files [][]byte) ([]byte, error) {
	totalLength := 0
	for _, file := range files {
		totalLength = len(file)
	}
	ret := make([]byte, totalLength+len(files)*16)
	curOutputLocation := 0
	curInputLocation := 0
	for _, file := range files {
		C.BroccoliNewBrotliFile(&state)
		for {
			var curData *byte
			if len(file) != 0 {
				curData = &file[curInputLocation]
			}
			avail_in := C.size_t(len(file))
			old_avail_in := avail_in
			avail_out := C.size_t(len(ret) - curOutputLocation)
			old_avail_out := avail_out
			concat_result := C.BrConcatStream(&state,
				&avail_in,
				(*C.uint8_t)(curData),
				&avail_out,
				(*C.uint8_t)(&ret[curOutputLocation]),
			)
			curInputLocation := int(avail_in - old_avail_in)
			curOutputLocation += int(avail_out - old_avail_out)
			if concat_result != C.BroccoliNeedsMoreOutput {
				if concat_result == C.BroccoliNeedsMoreInput { // done with this file
					if curInputLocation != len(file) {
						return nil, fmt.Errorf("Broccoli: NeedMoreInput returned but %d input avail",
							len(file)-curInputLocation)
					}
					break
				}
				return nil, fmt.Errorf("Broccoli Error Code: %v", concat_result)
			}
			if curOutputLocation == len(ret) { // if our estimate of 16 * num_files + size is bad
				ret = append(ret, make([]byte, 65536)...)
			}
		}
	}
	var concat_result C.BroccoliResult
	for {
		avail_out := C.size_t(len(ret) - curOutputLocation)
		if avail_out == 0 {
			ret = append(ret, make([]byte, 65536)...)
			avail_out = C.size_t(len(ret) - curOutputLocation)
		}
		old_avail_out := avail_out
		concat_result = C.BrConcatFinish(
			&state,
			&avail_out,
			(*C.uint8_t)(&ret[curOutputLocation]),
		)
		curOutputLocation += int(avail_out - old_avail_out)
		if concat_result != C.BroccoliNeedsMoreOutput {
			break
		}
	}
	if concat_result != C.BroccoliSuccess {
		return nil, fmt.Errorf("Broccoli Error Code: %v", concat_result)
	}
	return ret[:curOutputLocation], nil
}

func BroccoliConcat(files ...[]byte) ([]byte, error) {
	state := C.BroccoliCreateInstance()
	return broccoliConcat(state, files)
}
func BroccoliConcatWithWindowSize(windowSize uint8, files ...[]byte) ([]byte, error) {
	state := C.BroccoliCreateInstanceWithWindowSize(C.uint8_t(windowSize))
	return broccoliConcat(state, files)
}

func main() {
	decompress := false
	options := CompressionOptions{
		NumThreads: 1,
		Quality:    9.5,
		Catable:    true,
		Appendable: true,
		Magic:      true,
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
		if arg == "-cat" {
			toCat = os.Args[1:index]
			toCat = append(toCat, os.Args[index+1:]...)
			break
		}
	}
	if toCat != nil {
		files := make([]io.Reader, len(toCat))
		for index, fn := range toCat {
			var err error
			files[index], err = os.Open(fn)
			if err != nil {
				panic(err)
			}
		}
		_, err := io.Copy(os.Stdout, NewBroccoliConcatReader(files...))
		if err != nil {
			panic(err)
		}
		for _, file := range files {
			if readCloser, ok := file.(io.ReadCloser); ok {
				_ = readCloser.Close()
			}
		}
		return
	} else if useWriter {
		var writer io.Writer
		if decompress {
			writer = NewDecompressionWriter(
				os.Stdout,
			)
		} else {
			writer = NewMultiCompressionWriter(
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
			reader = NewDecompressionReader(
				os.Stdin,
			)
		} else {
			reader = NewMultiCompressionReader(
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
