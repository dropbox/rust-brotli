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

void simple_test() {
#ifndef NOSTDLIB
    const unsigned char brotli_file[] = {0x1b, 0x30, 0x00, 0xe0, 0x8d, 0xd4, 0x59, 0x2d, 0x39, 0x37, 0xb5, 0x02,
                                   0x48, 0x10, 0x95, 0x2a, 0x9a, 0xea, 0x42, 0x0e, 0x51, 0xa4, 0x16, 0xb9,
                                   0xcb, 0xf5, 0xf8, 0x5c, 0x64, 0xb9, 0x2f, 0xc9, 0x6a, 0x3f, 0xb1, 0xdc,
                                   0xa8, 0xe0, 0x35, 0x07};
    const unsigned char key[] = "THIS IS A TEST OF THE EMERGENCY BROADCAST SYSTEM";
    unsigned char output[sizeof(key) * 2];
    size_t decoded_size = sizeof(output) * 2;
    BrotliDecoderDecompress(sizeof(brotli_file), brotli_file, &decoded_size, output);
    assert(decoded_size == sizeof(key));
    assert(memcmp(output, key, sizeof(key) - 1) == 0);
    assert(output[sizeof(key) - 1] == '\n');
#endif
}

int main() {
    simple_test();
    BrotliDecoderState * state = BrotliDecoderCreateInstance(custom_alloc, custom_free, &custom_alloc_data);
    unsigned char ibuffer[4096];
    unsigned char obuffer[4096];
    size_t total_out = 0;
    BrotliDecoderResult rest;
    while(1) {
        size_t avail_in = fread(ibuffer, 1, sizeof(ibuffer), stdin);
        int is_eof = (avail_in == 0);
        const unsigned char *i_ptr = &ibuffer[0];
        while (1) {
            unsigned char *o_ptr = &obuffer[0];
            size_t avail_out = sizeof(obuffer);
            rest = BrotliDecoderDecompressStream(state, &avail_in, &i_ptr, &avail_out, &o_ptr, &total_out);
            if (o_ptr != &obuffer[0]) {
                size_t ret = fwrite(obuffer, 1, o_ptr - &obuffer[0], stdout);
                assert(ret == o_ptr - &obuffer[0]);
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
            exit(1);
        }
        if (rest == BROTLI_DECODER_RESULT_SUCCESS || rest == BROTLI_DECODER_RESULT_ERROR) {
            break;
        }
    }
    BrotliDecoderDestroyInstance(state);
}
