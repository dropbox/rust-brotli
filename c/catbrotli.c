#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include "brotli/broccoli.h"

void usage() {
    fprintf(stderr,
            "Usage: [-w<window_size>] filename0 filename1 filename2...");
}


int main(int argc, char**argv) {
    unsigned char window_size = 0;
    unsigned char has_window_size = 0;
    unsigned char double_dash = 0;
    size_t buffer_size = 4096;
    int i;
    if (argc == 1) {
        usage();
        return 1;
    }
    for (i = 1; i < argc; ++i) {
        if (strcmp(argv[i], "-help") == 0 && !double_dash) {
            usage();
            return 1;
        }
        if (strcmp(argv[i], "--help") == 0 && !double_dash) {
            usage();
            return 1;
        }
        if (strcmp(argv[i], "-h") == 0 && !double_dash) {
            usage();
            return 1;
        }
        if (strncmp(argv[i], "-w", 2) == 0 && !double_dash) {
            int j;
            has_window_size = 1;
            window_size = atoi(argv[i] + 2);
            for (j = i; j + 1 < argc; ++j) {
                argv[j] = argv[j+1];
            }
            --i;
            --argc;
            continue;
        }
        if (strncmp(argv[i], "-bs", 3) == 0 && !double_dash) {
            int j;
            buffer_size = atoi(argv[i] + 3);
            for (j = i; j + 1 < argc; ++j) {
                argv[j] = argv[j+1];
            }
            --i;
            --argc;
            continue;
        }
        if (strcmp(argv[i], "--") == 0) {
            int j;
            double_dash = 1;
            for (j = i; j + 1 < argc; ++j) {
                argv[j] = argv[j+1];
            }
            --i;
            --argc;
            continue;
        }
    }
    char ** filenames = &argv[1];
    unsigned char * ibuffer = (unsigned char*)malloc(buffer_size);
    unsigned char * obuffer = (unsigned char*)malloc(buffer_size);
    unsigned char* obuffer_ptr = obuffer;
    size_t avail_out = buffer_size;
    struct BroccoliState state;
    int i;
    if (has_window_size) {
        state = BroccoliCreateInstanceWithWindowSize(window_size);
    } else {
        state = BroccoliCreateInstance();
    }
    for (i = 1; i < argc; ++i) {
        BroccoliNewBrotliFile(&state);
        FILE * input_file = fopen(argv[i], "rb");
        if (!input_file) {
            fprintf(stderr, "Could not open %s\n", argv[i]);
            usage();
            return 1;
        }
        while(1) {
            size_t cur_read = fread(ibuffer, 1, buffer_size, input_file);
            const unsigned char *ibuffer_ptr = ibuffer;
            size_t avail_in = cur_read;
            if (cur_read == 0) {
                break;
            }
            while(1) {
                enum BroccoliResult res = BroccoliConcatStream(
                    &state,
                    &avail_in,
                    &ibuffer_ptr,
                    &avail_out,
                    &obuffer_ptr);
                if (res == BroccoliSuccess) {
                    abort();
                }
                if (res == BroccoliNeedsMoreInput) {
                    break;
                }
                if (res == BroccoliNeedsMoreOutput) {
                    fwrite(obuffer, 1, (obuffer_ptr - obuffer), stdout);
                    obuffer_ptr = obuffer;
                    avail_out = buffer_size;
                    continue;
                }
                abort();
            }
        }
    }
    while(1) {
        enum BroccoliResult res = BroccoliConcatFinish(
            &state,
            &avail_out,
            &obuffer_ptr);
        if (res == BroccoliNeedsMoreOutput) {
            fwrite(obuffer, 1, (obuffer_ptr - obuffer), stdout);
            obuffer_ptr = obuffer;
            avail_out = buffer_size;
            continue;
        }
        if (res == BroccoliNeedsMoreInput) {
            abort();
        }
        if (res == BroccoliSuccess) {
            fwrite(obuffer, 1, (obuffer_ptr - obuffer), stdout);
            break;
        }
        abort(); //failure
    }
    return 0;
}
