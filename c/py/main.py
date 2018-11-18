import ctypes
from ctypes import c_uint, pointer, POINTER, c_size_t, c_void_p, c_uint32, c_ubyte
class BrotliEncoderWorkPool(ctypes.Structure):
    pass
BrotliEncoderWorkPool= ctypes.POINTER(BrotliEncoderWorkPool)

brotli_library=ctypes.CDLL("../target/release/libbrotli_ffi.dylib")
_BrotliEncoderCreateWorkPool = brotli_library.BrotliEncoderCreateWorkPool
_BrotliEncoderCreateWorkPool.restype = POINTER(BrotliEncoderWorkPool)
_BrotliEncoderCompressWorkPool = brotli_library.BrotliEncoderCompressWorkPool
_BrotliEncoderCompressWorkPool.restype = c_uint32
BrotliEncoderMaxCompressedSizeMulti = brotli_library.BrotliEncoderMaxCompressedSizeMulti
BrotliEncoderMaxCompressedSizeMulti.restype = c_size_t
def BrotliEncoderCreateWorkPool(num_workers):
    return _BrotliEncoderCreateWorkPool(c_size_t(num_workers),
                                        c_void_p(),
                                        c_void_p(),
                                        c_void_p())

BrotliEncoderDestroyWorkPool = brotli_library.BrotliEncoderDestroyWorkPool
BrotliEncoderDestroyWorkPool.restype = None

BROTLI_PARAM_MODE = 0
# The main compression speed-density lever.
#
# The higher the quality, the slower the compression. Range is
# from ::BROTLI_MIN_QUALITY to ::BROTLI_MAX_QUALITY.
BROTLI_PARAM_QUALITY = 1
# Recommended sliding LZ77 window size.
#
# Encoder may reduce this value, e.g. if input is much smaller than
# window size.
#
# Window size is `(1 << value - 16`.
#
# Range is from ::BROTLI_MIN_WINDOW_BITS to ::BROTLI_MAX_WINDOW_BITS.
BROTLI_PARAM_LGWIN = 2
# Recommended input block size.
#
# Encoder may reduce this value, e.g. if input is much smaller than input
# block size.
#
# Range is from ::BROTLI_MIN_INPUT_BLOCK_BITS to
# ::BROTLI_MAX_INPUT_BLOCK_BITS.
#
# @note Bigger input block size allows better compression, but consumes more
#       memory. \n The rough formula of memory used for temporary input
#       storage is `3 << lgBlock`.
BROTLI_PARAM_LGBLOCK = 3
# Flag that affects usage of "literal context modeling" format feature.
#
# This flag is a "decoding-speed vs compression ratio" trade-off.
BROTLI_PARAM_DISABLE_LITERAL_CONTEXT_MODELING = 4
# Estimated total input size for all ::BrotliEncoderCompressStream calls.
#
# The default value is 0, which means that the total input size is unknown.
BROTLI_PARAM_SIZE_HINT = 5
# Flag that determines if "Large Window Brotli" is used.
BROTLI_PARAM_LARGE_WINDOW = 6
# Recommended number of postfix bits (NPOSTFIX).
#
# Encoder may change this value.
#
# Range is from 0 to ::BROTLI_MAX_NPOSTFIX.
BROTLI_PARAM_NPOSTFIX = 7
# Recommended number of direct distance codes (NDIRECT).
#
# Encoder may change this value.
#
# Range is from 0 to (15 << NPOSTFIX) in steps of (1 << NPOSTFIX).
BROTLI_PARAM_NDIRECT = 8
BROTLI_PARAM_Q9_5 = 150
BROTLI_METABLOCK_CALLBACK = 151
BROTLI_PARAM_STRIDE_DETECTION_QUALITY = 152
BROTLI_PARAM_HIGH_ENTROPY_DETECTION_QUALITY = 153
BROTLI_PARAM_LITERAL_BYTE_SCORE = 154
BROTLI_PARAM_CDF_ADAPTATION_DETECTION = 155
BROTLI_PARAM_PRIOR_BITMASK_DETECTION = 156
BROTLI_PARAM_SPEED = 157
BROTLI_PARAM_SPEED_MAX = 158
BROTLI_PARAM_CM_SPEED = 159
BROTLI_PARAM_CM_SPEED_MAX = 160
BROTLI_PARAM_SPEED_LOW = 161
BROTLI_PARAM_SPEED_LOW_MAX = 162
BROTLI_PARAM_CM_SPEED_LOW = 164
BROTLI_PARAM_CM_SPEED_LOW_MAX = 165
BROTLI_PARAM_AVOID_DISTANCE_PREFIX_SEARCH = 166
BROTLI_PARAM_CATABLE = 167
BROTLI_PARAM_APPENDABLE = 168
BROTLI_PARAM_MAGIC_NUMBER = 169

def BrotliEncoderCompressWorkPool(
        work_pool,
        input,
        compression_options_map={},
        num_threads=4,
        ):
    OptionsKeysArrayDecl = c_uint * len(compression_options_map)
    OptionsValuesArrayDecl = c_uint32 * len(compression_options_map)
    index = 0
    options_keys = OptionsKeysArrayDecl()
    options_values = OptionsValuesArrayDecl()
    for k, v in compression_options_map.iteritems():
        options_keys[index] = c_uint(k)
        options_values[index] = c_uint32(v)
        index += 1
    max_size = BrotliEncoderMaxCompressedSizeMulti(c_size_t(len(input)),
                                                   c_size_t(num_threads)) + 150
    EncodedArrayDecl = c_ubyte
    encoded = (c_ubyte  * max_size)()
    encoded_size = c_size_t(max_size)
    ret_code = brotli_library.BrotliEncoderCompressWorkPool(
        work_pool,
        c_size_t(len(compression_options_map)),
        options_keys,
        options_values,
        c_size_t(len(input)),
        input,
        pointer(encoded_size),
        pointer(encoded),
        c_size_t(num_threads),
        c_void_p(),
        c_void_p(),
        c_void_p(),
        )
    return bytearray(encoded[:encoded_size.value])

def BrotliEncoderCompress(
        input,
        compression_options_map={},
        num_threads=4,
        ):
    wp = BrotliEncoderCreateWorkPool(num_threads)
    ret = BrotliEncoderCompressWorkPool(wp, input, num_threads, compression_options_map)
    BrotliEncoderDestroyWorkPool(wp)
    return ret


def main():
    import sys
    if len(sys.argv) > 1:
        with open(sys.argv[1]) as f:
            data = f.read()
    else:
        data = sys.stdin.read()
    work_pool = BrotliEncoderCreateWorkPool(4)
    encoded = BrotliEncoderCompressWorkPool(work_pool, data, {
        BROTLI_PARAM_QUALITY:11,
        BROTLI_PARAM_Q9_5:0,
        BROTLI_PARAM_LGWIN: 16,
        BROTLI_PARAM_MAGIC_NUMBER: 0,
        BROTLI_PARAM_CATABLE: 0,
        BROTLI_PARAM_SIZE_HINT: len(data),
    },4 )
    sys.stdout.write(encoded)
    BrotliEncoderDestroyWorkPool(work_pool)
if __name__ == "__main__":
    main()
