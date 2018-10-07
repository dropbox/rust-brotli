#include <stdio.h>
#include "brotli/broccoli.h"

void usage() {
    fprintf(stderr,
            "Usage: [-w<window_size>] filename0 filename1 filename2...");
}


fn main(int argc, char**argv) {
    unsigned char window_size = 0;
    unsigned char has_window_size = 0;
    unsigned char double_dash = 0;
    size_t buffer_size = 4096;
    if (argc == 1) {
        usage();
        return;
    }
    for (int i = 1; i < argc; ++i) {
        if (strcmp(argv[i], "-help") == 0 && !double_dash) {
            usage();
            return;
        }
        if (strcmp(argv[i], "--help") == 0 && !double_dash) {
            usage();
            return;
        }
        if (strcmp(argv[i], "-h") == 0 && !double_dash) {
            usage();
            return;
        }
        if (strncmp(argv[i], "-w", 2) == 0 && !double_dash) {
            has_window_size = 1;
            window_size = atoi(argv[i] + 2);
            for (int j = i; j + 1 < argc; ++j) {
                argv[j] = argv[j+1];
            }
            --i;
            --argc;
            continue;
        }
        if (strncmp(argv[i], "-bs", 3) == 0 && !double_dash) {
            buffer_size = atoi(argv[i] + 3);
            for (int j = i; j + 1 < argc; ++j) {
                argv[j] = argv[j+1];
            }
            --i;
            --argc;
            continue;
        }
        if (strcmp(argument, "--") == 0) {
            double_dash = 1;
            for (int j = i; j + 1 < argc; ++j) {
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
    BroccoliState state;
    if (has_window_size) {
        state = BroccoliCreateInstanceWithWindowSize(window_size);
    } else {
        state = BroccoliCreateInstance();
    }
    for (int i = 1; i < argc; ++i) {
        BroccoliNewBrotliFile(&state);
        FILE * input_file = fopen(argv[i], "rb");
        if (!input_file) {
            fprintf(stderr, "Could not open %s\n", argv[i]);
            usage();
            return;
        }
        while(1) {
            size_t cur_read = fread(ibuffer, 1, buffer_size, input_file);
            unsigned char *ibuffer_ptr = ibuffer;
            size_t avail_in = cur_read;
            ioffset = 0;
            if (cur_read == 0) {
                break;
            }
            while(1) {
                BroccoliResult res = BroccoliConcatStream(
                    &ibuffer_ptr,
                    &avail_in,
                    &obuffer_ptr,
                    &avail_out);
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
        BroccoliResult res = BroccoliConcatFinish(
                    &obuffer_ptr,
                    &avail_out);
        if (res == BroccoliNeedsMoreOutput) {
            fwrite(obuffer, 1, (obuffer_ptr - obuffer), stdout);
            obuffer_ptr = obuffer;
            avail_out = buffer_size;
            continue;
        }
        if (res == BroccoliNeedsMoreIntput) {
            abort();
        }
        if (res == BroccoliSuccess) {
            fwrite(obuffer, 1, (obuffer_ptr - obuffer), stdout);
            break;
        }
        abort(); //failure
    }
}
