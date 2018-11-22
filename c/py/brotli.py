import ctypes
import sys

from ctypes import c_uint, c_int, pointer, POINTER, c_size_t, c_void_p, c_uint32, c_ubyte, c_char_p, byref
class BrotliEncoderWorkPool(ctypes.Structure):
    pass
BrotliEncoderWorkPool= ctypes.POINTER(BrotliEncoderWorkPool)
try:
    brotli_library=ctypes.CDLL("../target/release/libbrotli_ffi.dylib")
except OSError:
    try:
        brotli_library=ctypes.CDLL("../target/release/libbrotli_ffi.so")
    except:
        try:
            brotli_library=ctypes.CDLL("target/release/libbrotli_ffi.dylib")
        except OSError:
            brotli_library=ctypes.CDLL("target/release/libbrotli_ffi.so")

_BrotliEncoderCreateWorkPool = brotli_library.BrotliEncoderCreateWorkPool
_BrotliEncoderCreateWorkPool.restype = POINTER(BrotliEncoderWorkPool)
_BrotliEncoderCompressWorkPool = brotli_library.BrotliEncoderCompressWorkPool
_BrotliEncoderCompressWorkPool.restype = c_uint32
class BrotliDecoderState(ctypes.Structure):
    pass
class BrotliDecompressorException(Exception):
    pass
class BrotliDecoderReturnInfo(ctypes.Structure):
    _fields_ = [('decoded_size', c_size_t),
                ('error_string', c_ubyte * 256),
                ('result', c_int),
                ('error_code', c_int),
    ]
BrotliDecoderDecompressWithReturnInfo = brotli_library.BrotliDecoderDecompressWithReturnInfo
BrotliDecoderDecompressWithReturnInfo.restype = BrotliDecoderReturnInfo
_BrotliDecoderCreateInstance = brotli_library.BrotliDecoderCreateInstance
_BrotliDecoderCreateInstance.restype = POINTER(BrotliDecoderState)
_BrotliDecoderDestroyInstance = brotli_library.BrotliDecoderDestroyInstance
_BrotliDecoderDestroyInstance.restype = None
BrotliEncoderMaxCompressedSizeMulti = brotli_library.BrotliEncoderMaxCompressedSizeMulti
BrotliEncoderMaxCompressedSizeMulti.restype = c_size_t
_BrotliDecoderDecompressStream = brotli_library.BrotliDecoderDecompressStream
_BrotliDecoderDecompressStream.restype = int

_BrotliDecoderGetErrorString = brotli_library.BrotliDecoderGetErrorString
_BrotliDecoderGetErrorString.restype = c_char_p 


_BrotliEncoderVersion = brotli_library.BrotliEncoderVersion
_BrotliEncoderVersion.restype = c_uint32

_BrotliDecoderVersion = brotli_library.BrotliDecoderVersion
_BrotliDecoderVersion.restype = c_uint32

def BrotliEncoderVersion():
    # type: () -> int
    return _BrotliEncoderVersion()

def BrotliDecoderVersion():
    # type: () -> int
    return _BrotliDecoderVersion()

BROTLI_DECODER_RESULT_ERROR = 0
BROTLI_DECODER_RESULT_SUCCESS = 1
BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT = 2
BROTLI_DECODER_RESULT_NEEDS_MORE_OUTPUT = 3

def BrotliEncoderCreateWorkPool(num_workers):
    return _BrotliEncoderCreateWorkPool(c_size_t(num_workers),
                                        c_void_p(),
                                        c_void_p(),
                                        c_void_p())

BrotliEncoderDestroyWorkPool = brotli_library.BrotliEncoderDestroyWorkPool
BrotliEncoderDestroyWorkPool.restype = None
class BrotliCompressorException(Exception):
    pass

def BrotliEncoderCompressWorkPool(
        work_pool,
        any_input,
        compression_options_map={},
        num_threads=4,
        ):
    input = _fix_ctype_input_arrays(any_input)
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
                                                   c_size_t(num_threads))
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
        byref(encoded_size),
        byref(encoded),
        c_size_t(num_threads),
        c_void_p(),
        c_void_p(),
        c_void_p(),
        )
    if ret_code == 0:
        raise BrotliCompressorException("Insufficient space "
                                        + str(max_size)
                                        + " to compress "
                                        + str(len(input))
                                        + " bytes with "
                                        + str(num_threads)
                                        + " threads")
    return bytearray(encoded[:encoded_size.value])

def _fix_ctype_input_arrays(any_input):
    if type(any_input) == memoryview:
        return any_input.tobytes()
    if type(any_input) != str and type(any_input) != bytes:
        try:
            return (c_ubyte * len(any_input)).from_buffer(any_input)
        except Exception:
            pass
    return any_input

def BrotliDecode(any_input, expected_size=4096 * 1024, max_expected_size = 256 * 1024 * 1024):
    input = _fix_ctype_input_arrays(any_input)
    while True:
        decoded_size = c_size_t(expected_size)
        decoded = (c_ubyte * decoded_size.value)()

        res = BrotliDecoderDecompressWithReturnInfo(len(input),
                                                    input,
                                                    decoded_size,
                                                    byref(decoded))
        if res.result == BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT:
            raise BrotliDecompressorException("EarlyEOF")
        elif res.result == BROTLI_DECODER_RESULT_NEEDS_MORE_OUTPUT:
            expected_size *= 2
            if expected_size > max_expected_size:
                raise BrotliDecompressorException("Brotli file > " + str(max_expected_size) + " or corrupt brotli file")
        elif res.result == BROTLI_DECODER_RESULT_SUCCESS:
            return bytearray(decoded[:res.decoded_size])
        else:
            raise BrotliDecompressorException(''.join(chr(x) for x in res.error_string))


def BrotliCompress(
        any_input,
        compression_options_map={},
        num_threads=4,
        ):
    input = _fix_ctype_input_arrays(any_input)
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
                                                   c_size_t(num_threads))
    EncodedArrayDecl = c_ubyte
    encoded = (c_ubyte  * max_size)()
    encoded_size = c_size_t(max_size)
    ret_code = brotli_library.BrotliEncoderCompressMulti(
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
        c_void_p())
    if ret_code == 0:
        raise BrotliCompressorException("Insufficient space "
                                        + str(max_size)
                                        + " to compress "
                                        + str(len(input))
                                        + " bytes with "
                                        + str(num_threads)
                                        + " threads")
    return bytearray(encoded[:encoded_size.value])






######################### CONSTANTS ###############################

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

#simple test binary
def main(args):
    work_pool = False
    decompress = False
    raw = False
    for (index, arg) in enumerate(args):
        if arg == '-workpool':
            work_pool = True
            args = args[:index] + args[index + 1:]
    for (index, arg) in enumerate(args):
        if arg == '-d':
            decompress = True
            args = args[:index] + args[index + 1:]
    for (index, arg) in enumerate(args):
        if arg == '-raw':
            raw = True
            args = args[:index] + args[index + 1:]
    if len(args) > 0:
        if raw:
            data = args[0]
        else:
            with open(args[0]) as f:
                data = f.read()
    else:
        data = sys.stdin.read()
    if decompress and work_pool:
        processed = BrotliDecode(data,2)
    elif decompress:
        processed = BrotliDecode(data)
    elif work_pool:
        work_pool = BrotliEncoderCreateWorkPool(4)
        processed = BrotliEncoderCompressWorkPool(work_pool, data, {
            BROTLI_PARAM_QUALITY:11,
            BROTLI_PARAM_Q9_5:0,
            BROTLI_PARAM_LGWIN: 16,
            BROTLI_PARAM_MAGIC_NUMBER: 0,
            BROTLI_PARAM_CATABLE: 0,
            BROTLI_PARAM_SIZE_HINT: len(data),
        },4 )
        BrotliEncoderDestroyWorkPool(work_pool)
    else:
        processed = BrotliCompress(data, {
            BROTLI_PARAM_QUALITY:11,
            BROTLI_PARAM_Q9_5:0,
            BROTLI_PARAM_LGWIN: 16,
            BROTLI_PARAM_MAGIC_NUMBER: 0,
            BROTLI_PARAM_CATABLE: 0,
            BROTLI_PARAM_SIZE_HINT: len(data),
        },4 )
    sys.stdout.write(processed)

if __name__ == "__main__":
    main(sys.argv[1:])

