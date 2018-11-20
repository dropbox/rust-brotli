#include "encode.h"


/**
 * Opaque structure that holds workpool thrads
 *
 * Allocated and initialized with ::BrotliEncoderCreateWorkPool.
 * Cleaned up and deallocated with ::BrotliEncoderDestroyWorkPool.
 */
typedef struct BrotliEncoderWorkPoolStruct BrotliEncoderWorkPool;

/**
 * Performs one-shot memory-to-memory compression.
 *
 * Compresses the data in @p input_buffer into @p encoded_buffer, and sets
 * @p *encoded_size to the compressed length.
 *
 * @note If ::BrotliEncoderMaxCompressedSize(@p input_size) returns non-zero
 *       value, then output is guaranteed to be no longer than that.
 *
 * @param num_params indicates how long both the param_keys and param_values arrays will be
 * @param param_keys is an array of BrotliEncoderParameters that must be the same length as param_values
 * @param param_values is an array of uint32_t and for each one of these, the matching param_keys will be changed to this value
 * @param input_size size of @p input_buffer
 * @param input_buffer input data buffer with at least @p input_size
 *        addressable bytes
 * @param[in, out] encoded_size @b in: size of @p encoded_buffer; \n
 *                 @b out: length of compressed data written to
 *                 @p encoded_buffer, or @c 0 if compression fails
 * @param encoded_buffer compressed data destination buffer
 * @param desired_num_threads is an integer about the number of threads to spawn to encode the input_buffer
 * @param alloc_func is an optional allocator that will be called for any allocations. If null, builtin malloc will be used
 * @param free_func is an optional allocator that will be called for any frees. If null, builtin free will be used
 * @param alloc_opaque_per_thread is an opaque function that will be passed into both alloc and free.
 *                                each thread will have an independent allocator and will pass its own opaque. So this array must
 *                                either be entirely null (and the above functions also be null) or sized to the desired_num_threads
 * @returns ::BROTLI_FALSE in case of compression error
 * @returns ::BROTLI_FALSE if output buffer is too small
 * @returns ::BROTLI_TRUE otherwise
 */
BROTLI_ENC_API int32_t BrotliEncoderCompressMulti(
    size_t num_params,
    const BrotliEncoderParameter* param_keys,
    const uint32_t* param_values,
    size_t input_size,
    const uint8_t *input_buffer,
    size_t *encoded_size,
    uint8_t *encoded,
    size_t desired_num_threads,
    brotli_alloc_func alloc_func, brotli_free_func free_func,
    void** alloc_opaque_per_thread);



BROTLI_ENC_API size_t BrotliEncoderMaxCompressedSizeMulti(size_t input_size, size_t num_threads);

/**
 * Creates an instance of ::BrotliEncoderWorkPool and initializes it, spawning num_threads threads
 *
 * @p alloc_func and @p free_func @b MUST be both zero or both non-zero. In the
 * case they are both zero, default memory allocators are used. @p opaque is
 * passed to @p alloc_func and @p free_func when they are called. @p free_func
 * has to return without doing anything when asked to free a NULL pointer.
 *
 * @param alloc_func custom memory allocation function
 * @param free_func custom memory free function
 * @param opaque custom memory manager handle
 * @returns @c 0 if instance can not be allocated or initialized
 * @returns pointer to initialized ::BrotliEncoderWorkPool otherwise
 */
BROTLI_ENC_API BrotliEncoderWorkPool* BrotliEncoderCreateWorkPool(
    size_t num_threads,
    brotli_alloc_func alloc_func,
    brotli_free_func free_func,
    void** alloc_opaque_per_thread);



/**
 * Deinitializes and frees ::BrotliEncoderWorkPool instance.
 *
 * @param state work pool instance to be cleaned up and deallocated
 */
BROTLI_ENC_API void BrotliEncoderDestroyWorkPool(BrotliEncoderWorkPool* work_pool);


/**
 * Performs one-shot memory-to-memory compression.
 *
 * Compresses the data in @p input_buffer into @p encoded_buffer, and sets
 * @p *encoded_size to the compressed length. Runs compression using existing threads spawned in work_pool
 *
 * @note If ::BrotliEncoderMaxCompressedSize(@p input_size) returns non-zero
 *       value, then output is guaranteed to be no longer than that.
 *
 * @param num_params indicates how long both the param_keys and param_values arrays will be
 * @param param_keys is an array of BrotliEncoderParameters that must be the same length as param_values
 * @param param_values is an array of uint32_t and for each one of these, the matching param_keys will be changed to this value
 * @param input_size size of @p input_buffer
 * @param input_buffer input data buffer with at least @p input_size
 *        addressable bytes
 * @param[in, out] encoded_size @b in: size of @p encoded_buffer; \n
 *                 @b out: length of compressed data written to
 *                 @p encoded_buffer, or @c 0 if compression fails
 * @param encoded_buffer compressed data destination buffer
 * @param desired_num_threads is an integer about the number of compression jobs the file should be split into
 * @param alloc_func is an optional allocator that will be called for any allocations. If null, builtin malloc will be used
 * @param free_func is an optional allocator that will be called for any frees. If null, builtin free will be used
 * @param alloc_opaque_per_thread is an opaque function that will be passed into both alloc and free.
 *                                each thread will have an independent allocator and will pass its own opaque. So this array must
 *                                either be entirely null (and the above functions also be null) or sized to the desired_num_threads
 * @returns ::BROTLI_FALSE in case of compression error
 * @returns ::BROTLI_FALSE if output buffer is too small
 * @returns ::BROTLI_TRUE otherwise
 */
BROTLI_ENC_API int32_t BrotliEncoderCompressWorkPool(
    BrotliEncoderWorkPool *work_pool,
    size_t num_params,
    const BrotliEncoderParameter* param_keys,
    const uint32_t* param_values,
    size_t input_size,
    const uint8_t *input_buffer,
    size_t *encoded_size,
    uint8_t *encoded,
    size_t desired_num_threads,
    brotli_alloc_func alloc_func, brotli_free_func free_func,
    void** alloc_opaque_per_thread);


