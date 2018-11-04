#include "brotli/decode.h"
#include <stdlib.h>
#include <stdio.h>
#include <assert.h>
#include <string.h>
int custom_alloc_data = 0;
void * custom_alloc(void*opaque, size_t size) {
    assert(opaque == &custom_alloc_data);
    return malloc(size);
}
void custom_free(void*opaque, void* addr) {
    assert(opaque == &custom_alloc_data);
    free(addr);
}

int main(int argc, char **argv) {
    FILE * fp = argc > 1 ? fopen(argv[1], "rb") : stdin;
    BrotliDecoderState * state = BrotliDecoderCreateInstance(custom_alloc, custom_free, &custom_alloc_data);
    unsigned char ibuffer[4096];
    unsigned char obuffer[4096];
    size_t total_out = 0;
    BrotliDecoderResult rest;
    while(1) {
        size_t avail_in = fread(ibuffer, 1, sizeof(ibuffer), fp);
        int is_eof = (avail_in == 0);
        const unsigned char *i_ptr = &ibuffer[0];
        while (1) {
            unsigned char *o_ptr = &obuffer[0];
            size_t avail_out = sizeof(obuffer);
            rest = BrotliDecoderDecompressStream(state, &avail_in, &i_ptr, &avail_out, &o_ptr, &total_out);
            if (o_ptr != &obuffer[0]) {
                // don't actually write
            }
            if (rest == BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT) {
                break;
            }
            if (rest == BROTLI_DECODER_RESULT_SUCCESS || rest == BROTLI_DECODER_RESULT_ERROR) {
                break;
            }
        }
        if (rest == BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT && is_eof) {
            fprintf(stderr, "Unexpected EOF\n");
            break;
        }
        if (rest == BROTLI_DECODER_RESULT_SUCCESS) {
            break;
        }
        if (rest == BROTLI_DECODER_RESULT_ERROR) {
            fprintf(stderr, "Error: %s\n", BrotliDecoderGetErrorString(state));
            if (BrotliDecoderGetErrorCode(state) == BROTLI_DECODER_ERROR_UNREACHABLE) {
                abort(); // possibly a stack trace
            }
            break;
        }
    }
    BrotliDecoderDestroyInstance(state);
    if (rest == BROTLI_DECODER_RESULT_ERROR) {
        fprintf(stderr, "Not a valid brotli file\n");
    }
}
