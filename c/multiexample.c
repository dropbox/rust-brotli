#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <assert.h>
#ifndef _WIN32
#include <unistd.h>
#endif
#include "brotli/multiencode.h"
#include "brotli/decode.h"
#include "arg.h"
#include "custom_alloc.h"
#include "vec_u8.h"
const unsigned char example[]=
    "Mary had a little lamb. Its fleece was white as snow.\n"
    "And every where that Mary went, the lamb was sure to go.\n"
    "It followed her to school one day which was against the rule.\n"
    "It made the children laugh and play to see a lamb at sch00l!\n\n\n\n"
    "0 1 1 2 3 5 8 13 21 34 55 89 144 233 377 610 987 1597 2584 4181 6765\n"
    "\x11\x99\x2f\xfc\xfe\xef\xff\xd8\xfd\x9c\x43"
    "Additional testing characters here";

#define MAX_THREADS 64
#define MAX_ARGS 1024
#define BUF_SIZE 65536
int32_t compress(const unsigned char *data, size_t len, struct VecU8 *ret_buffer,
                      int argc, char** argv) {
    unsigned char buf[BUF_SIZE];
    BrotliEncoderParameter param_keys[MAX_ARGS];
    uint32_t param_values[MAX_ARGS];
    void * opaque_per_thread[MAX_THREADS];
    size_t num_threads = 1;
    size_t num_params = set_options(param_keys, param_values, argc > MAX_ARGS ? MAX_ARGS : argc, argv, len, &num_threads);
    BrotliEncoderWorkPool *work_pool = BrotliEncoderCreateWorkPool(num_threads != 0 ? num_threads - 1 : 0, custom_malloc, custom_free, custom_alloc_opaque);
    size_t out_len = BrotliEncoderMaxCompressedSizeMulti(len, num_threads);
    int32_t ret;
    {
        size_t i;
        for (i = 0; i < MAX_THREADS; i+=1) {
            opaque_per_thread[i] = custom_alloc_opaque;
        }
    }
    reinit_vec_u8(ret_buffer, out_len);
    ret = BrotliEncoderCompressWorkPool(
        work_pool,
        num_params,
        param_keys,
        param_values,
        len,
        data,
        &out_len,
        ret_buffer->data,
        num_threads,
        custom_malloc, custom_free, custom_malloc ? custom_alloc_opaque : NULL);
        
    BrotliEncoderDestroyWorkPool(work_pool);
    trunc_vec_u8(ret_buffer, out_len);
    return ret;
}

int32_t compress_immediate_thread_spawn(const unsigned char *data, size_t len, struct VecU8 *ret_buffer,
                                        int argc, char** argv) {
    unsigned char buf[BUF_SIZE];
    BrotliEncoderParameter param_keys[MAX_ARGS];
    uint32_t param_values[MAX_ARGS];
    void * opaque_per_thread[MAX_THREADS];
    size_t num_threads = 1;
    size_t num_params = set_options(param_keys, param_values, argc > MAX_ARGS ? MAX_ARGS : argc, argv, len, &num_threads);
    size_t out_len = BrotliEncoderMaxCompressedSizeMulti(len, num_threads);
    int32_t ret;
    {
        size_t i;
        for (i = 0; i < MAX_THREADS; i+=1) {
            opaque_per_thread[i] = custom_alloc_opaque;
        }
    }
    reinit_vec_u8(ret_buffer, out_len);
    ret = BrotliEncoderCompressMulti(
        num_params,
        param_keys,
        param_values,
        len,
        data,
        &out_len,
        ret_buffer->data,
        num_threads,
        custom_malloc, custom_free, custom_malloc ? custom_alloc_opaque : NULL);

    trunc_vec_u8(ret_buffer, out_len);
    return ret;
}

BrotliDecoderResult decompress(const unsigned char *data, size_t len, struct VecU8 *ret_buffer) {
    unsigned char buf[BUF_SIZE];
    size_t total_out = 0;
    BrotliDecoderState *state = BrotliDecoderCreateInstance(custom_malloc, custom_free, custom_alloc_opaque);
    BrotliDecoderResult res;
    do {
        size_t read_offset = 0;
        uint8_t *next_out = &buf[0];
        size_t avail_out = BUF_SIZE;
        res = BrotliDecoderDecompressStream(state,
                                            &len, &data,
                                            &avail_out, &next_out, &total_out);
        if (res == BROTLI_DECODER_RESULT_ERROR || (res == BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT && len == 0)) {
            BrotliDecoderDestroyInstance(state);
            return res;
        }
        push_vec_u8(ret_buffer, buf, next_out - buf);
    } while (res != BROTLI_DECODER_RESULT_SUCCESS);
    BrotliDecoderDestroyInstance(state);
    return BROTLI_DECODER_RESULT_SUCCESS;
}

int main(int argc, char**argv) {
    custom_free_f(&use_fake_malloc, memset(custom_malloc_f(&use_fake_malloc, 127), 0x7e, 127));
    if (getenv("NO_MALLOC")) {
        custom_alloc_opaque = &use_fake_malloc;
    }
    if (getenv("RUST_MALLOC")) {
        custom_alloc_opaque = NULL;
        custom_malloc = NULL;
        custom_free = NULL;
    }
    const unsigned char* data = example;
    size_t len = sizeof(example);
    unsigned char* to_free = NULL;
    if (find_first_arg(argc, argv)) {
        FILE * fp = fopen(find_first_arg(argc, argv), "rb");
        if (fp != NULL) {
            size_t ret;
            (void)fseek(fp, 0, SEEK_END);
            len = ftell(fp);
            (void)fseek(fp, 0, SEEK_SET);
            to_free = malloc(len);
            ret = fread(to_free, 1, len, fp);
            if  (ret == 0) {
                return -1;
            }
            data = to_free;
            (void)fclose(fp);
        }
    }
    {
        struct VecU8 brotli_file = new_vec_u8();
        struct VecU8 rt_file = new_vec_u8();
        BrotliDecoderResult dres;
        int32_t res;
        if (getenv("NO_WORK_POOL")) {
            res = compress_immediate_thread_spawn(data, len, &brotli_file, argc, argv);
        } else {
            res = compress(data, len, &brotli_file, argc, argv);
        }
        if (res != 1) {
            fprintf(stderr, "Failed to compress code:%d\n", (int) res);
            abort();
        }
        dres = decompress(brotli_file.data, brotli_file.size, &rt_file);
        if (dres != BROTLI_DECODER_RESULT_SUCCESS) {
            fprintf(stderr, "Failed to decompress file size %d code:%d\n", (int) brotli_file.size, (int)res);
            abort();
        }
        if (rt_file.size != len) {
            FILE * fp = fopen("/tmp/fail.rt", "wb");
            fwrite(rt_file.data, 1, rt_file.size, fp);
            fclose(fp);
            fp = fopen("/tmp/fail.dv", "wb");
            fwrite(brotli_file.data, 1, brotli_file.size, fp);
            fclose(fp);
            fp = fopen("/tmp/fail.or", "wb");
            fwrite(data, 1, len, fp);
            fclose(fp);
            fprintf(stderr, "Decompressed file size %ld != %ld\n", (long) rt_file.size, (long)len);
            abort();
        }
        if (memcmp(rt_file.data, data, len) != 0) {
            fprintf(stderr, "Roundtrip Contents mismatch\n");
            abort();
        }
#ifdef _WIN32
        printf("File length %ld reduced to %ld, %0.2f%%\n",
               (long)len, (long)brotli_file.size,(double)brotli_file.size * 100.0 / (double)len);
#else
        char buf[512];
        int ret;
        ret = write(1, "File length ", strlen("File Length "));
        if (ret <= 0) {
            return ret;
        }
        custom_atoi(buf, len);
        ret = write(1, buf, strlen(buf));
        if (ret <= 0) {
            return ret;
        }
        ret = write(1, " reduced to ", strlen(" reduced to "));
        if (ret <= 0) {
            return ret;
        }
        custom_atoi(buf, brotli_file.size);
        ret = write(1, buf, strlen(buf));
        if (ret <= 0) {
            return ret;
        }
        ret = write(1, ", ", strlen(", "));
        if (ret <= 0) {
            return ret;
        }
        custom_atoi(buf, brotli_file.size * 100 / len);
        ret = write(1, buf, strlen(buf));
        if (ret <= 0) {
            return ret;
        }
        ret = write(1, ".", strlen("."));
        if (ret <= 0) {
            return ret;
        }
        custom_atoi(buf, ((brotli_file.size * 1000000 + len/2)/ len) % 10000 + 10000);
        ret = write(1, buf + 1, strlen(buf) - 1);
        if (ret <= 0) {
            return ret;
        }
        ret = write(1, "%\n", strlen("%\n"));
        if (ret <= 0) {
            return ret;
        }
#endif
        release_vec_u8(&brotli_file);
        release_vec_u8(&rt_file);
    }
    if (to_free != NULL) {
        free(to_free);
    }
    return 0;
}
