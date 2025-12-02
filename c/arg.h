#ifndef _WIN32
#include <unistd.h>
#endif
char * find_first_arg(int argc, char**argv, long long int *truncation_location) {
    int i;
    for (i = 1; i < argc; ++i) {
        if (strstr(argv[i], "-trunc=") == argv[i]) {
            *truncation_location = strtoll(argv[i] + 7, NULL, 10);
        }
    }
    for (i = 1; i < argc; ++i) {
        if (argv[i][0] != '-') {
            return argv[i];
        }
    }
    return NULL;
}
size_t set_options(BrotliEncoderParameter *out_encoder_param_keys,
                   uint32_t *out_encoder_param_values,
                   int argc, char **argv,
                   size_t size_hint,
                   size_t* out_num_threads) {
    int i;
    size_t ret = 0;
    int used_cm = 0;
    out_encoder_param_keys[ret] = BROTLI_PARAM_SIZE_HINT;
    out_encoder_param_values[ret] = (uint32_t)size_hint;
    ret += 1;
    for (i = 1; i < argc; ++i) {
        if (strstr(argv[i], "-q") == argv[i]) {
            out_encoder_param_keys[ret] = BROTLI_PARAM_QUALITY;
            out_encoder_param_values[ret] = atoi(argv[i] + 2);
            ret += 1;
        }
        if (strstr(argv[i], "-p") == argv[i]) {
            out_encoder_param_keys[ret] = BROTLI_PARAM_LITERAL_BYTE_SCORE;
            out_encoder_param_values[ret] = atoi(argv[i] + 2);
            ret += 1;
        }
        if (strstr(argv[i], "-l") == argv[i]) {
            out_encoder_param_keys[ret] = BROTLI_PARAM_LGBLOCK;
            out_encoder_param_values[ret] = atoi(argv[i] + 2);
            ret += 1;
        }
        if (strstr(argv[i], "-w") == argv[i]) {
            out_encoder_param_keys[ret] = BROTLI_PARAM_LGWIN;
            out_encoder_param_values[ret] = atoi(argv[i] + 2);
            ret += 1;
        }
        if (strstr(argv[i], "-j") == argv[i]) {
            *out_num_threads = atoi(argv[i] + 2);
        }
        if (strstr(argv[i], "-timeout=") == argv[i]) {
            static int has_been_set = 0;
            int deadline = atoi(argv[i] + 9);
#ifndef _WIN32
            if (!has_been_set) {
                (void)alarm(deadline);
                has_been_set = 1;
            }
#endif
        }
        if (strstr(argv[i], "-m") == argv[i]) {
            out_encoder_param_keys[ret] = BROTLI_PARAM_MAGIC_NUMBER;
            out_encoder_param_values[ret] = 1;
            ret += 1;
        }
        if (strstr(argv[i], "-catable") == argv[i]) {
            out_encoder_param_keys[ret] = BROTLI_PARAM_CATABLE;
            out_encoder_param_values[ret] = 1;
            ret += 1;
        }
        if (strstr(argv[i], "-appendable") == argv[i]) {
            out_encoder_param_keys[ret] = BROTLI_PARAM_APPENDABLE;
            out_encoder_param_values[ret] = 1;
            ret += 1;
        }
        if (strstr(argv[i], "-bytealign") == argv[i]) {
            out_encoder_param_keys[ret] = BROTLI_PARAM_BYTE_ALIGN;
            out_encoder_param_values[ret] = 1;
            ret += 1;
        }
        if (strstr(argv[i], "-bare") == argv[i]) {
            out_encoder_param_keys[ret] = BROTLI_PARAM_BARE_STREAM;
            out_encoder_param_values[ret] = 1;
            ret += 1;
        }
    }
    return ret;
}
