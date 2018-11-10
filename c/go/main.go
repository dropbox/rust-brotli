package main
/*
#cgo CFLAGS: -I..
#cgo LDFLAGS: target/debug/libbrotli_ffi.a
#include <brotli/encode.h>
#include <brotli/multiencode.h>
*/
import "C"
import (
"errors"
"io"
"os"
)


type CompressionOptions struct {
   NumThreads int
   Quality float32
}

type MultiCompressionReader struct {
   options CompressionOptions
   buffer []byte
   output []byte
   upstream io.Reader
}

func NewMultiCompressionReader(
    upstream io.Reader,
    options CompressionOptions,
) MultiCompressionReader {
   return MultiCompressionReader{options:options, upstream:upstream}
}

func (mself *MultiCompressionReader) Read(data []byte) (int, error) {
    for mself.upstream != nil {
       size, err := mself.upstream.Read(data)
       mself.buffer = append(mself.buffer, data[:size]...)
       if err == io.EOF {
          downstream = nil
          break
       }
       var bufferAddr *C.uint8_t
       if len(mself.buffer) != 0 {
          bufferAddr = (*C.uint8_t)(&mself.buffer[0])
       }
       self.output = make([]byte, int(C.BrotliEncoderMaxCompressedSizeMulti(
          C.size_t(len(mself.buffer)), C.size_t(mself.options.NumThreads))))
       parameters, values, numParams := makeCompressionOptionsSteam(
           options,
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
          nil, nil, nil,
       );
       if ret == 0 {
          return errors.New("Compression failed")
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
    qualityParams := []C.BrotliEncoderParameter {qualityParam}
    values := []C.uint32_t{C.uint32_t(mself.options.Quality)}
    return &qualityParams[0], &values[0], 1  
}

type MultiCompressionWriter struct {
   options CompressionOptions
   buffer []byte
   downstream io.Writer
}

func NewMultiCompressionWriter(
    downstream io.Writer,
    options CompressionOptions,
) MultiCompressionWriter {
   return MultiCompressionWriter{options: options, downstream: downstream}
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
    parameters, values, numParams := makeCompressionOptionsSteam(
        options,
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
      nil, nil, nil);
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


func main() {
   writer := false
   if writer {
      compressedWriter := NewMultiCompressionWriter(
         os.Stdout,
         CompressionOptions{
             NumThreads: 8,
             Quality: 11,
         },
      )
    for {
       var buffer [65536]byte
       count, err := os.Stdin.Read(buffer[:])
       if err == io.EOF || count == 0 {
           err = compressedWriter.Close()
           if err != nil {
              panic(err)
           }
          return
       }
       if err != nil {
          panic(err)
       }
       _, err = compressedWriter.Write(buffer[:count])
       if err != nil {
        panic(err)
       }
    }
  } else {
    compressedReader := NewMultiCompressionReader(
      os.Stdin,
         CompressionOptions{
             NumThreads: 8,
             Quality: 9.5,
         },
    )
    for {
       var buffer [65536] byte
       size, err := compressedReader.Read(buffer[:])
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