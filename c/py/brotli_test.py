import unittest
import os
from .brotli import *
from .testdata import *
class TestBrotliLibrary(unittest.TestCase):
    def setUp(self):
        self.test_data = make_test_data(4096 * 1024)
    def test_version(self):
        assert BrotliDecoderVersion()
        assert BrotliEncoderVersion()

    def test_wp_rt(self):
        wp = BrotliEncoderCreateWorkPool(8)
        output = BrotliEncoderCompressWorkPool(wp,
                                      self.test_data,
                                      {
                                          BROTLI_PARAM_QUALITY:5,
                                          BROTLI_PARAM_CATABLE:1,
                                          BROTLI_PARAM_MAGIC_NUMBER:1,
                                          },
                                      8)
        BrotliEncoderDestroyWorkPool(wp)
        rt = BrotliDecode(output)
        assert rt == self.test_data
        assert len(output) < 1024 * 1024

    def test_header(self):
        data = ''.join(chr(x) for x in [
            0x6b, 0x1d, 0x00, 0xe1, 0x97, 0x81, 0x01, 0xe8, 0x99, 0xf4, 0x01, 0x08,
            0x00, 0x08, 0x79, 0x0a, 0x2c, 0x67, 0xe8, 0x81, 0x5f, 0x22, 0x2f, 0x1e,
            0x8b, 0x08, 0x3e, 0x09, 0x7a, 0x06
        ])
        parsed_header = BrotliParseHeader(data)
        assert parsed_header
        version, size = parsed_header
        assert size == 4001000
        assert version == 1

    def test_rt(self):
        output = BrotliCompress(self.test_data,
                                {
                                    BROTLI_PARAM_QUALITY:5,
                                    BROTLI_PARAM_CATABLE:1,
                                    BROTLI_PARAM_MAGIC_NUMBER:1,
                                },
                                8)
        rt = BrotliDecode(output)
        assert rt == self.test_data
        assert len(output) < 1024 * 1024
    def test_tiny_alloc(self):
        output = BrotliCompress(self.test_data,
                                {
                                    BROTLI_PARAM_QUALITY:5,
                                    BROTLI_PARAM_CATABLE:1,
                                    BROTLI_PARAM_MAGIC_NUMBER:1,
                                },
                                8)
        rt = BrotliDecode(output, 2)
        assert rt == self.test_data
        assert len(output) < 1024 * 1024
    def test_single_thread(self):
        output = BrotliCompress(self.test_data,
                                {
                                    BROTLI_PARAM_QUALITY:5,
                                    BROTLI_PARAM_CATABLE:1,
                                    BROTLI_PARAM_MAGIC_NUMBER:1,
                                },
                                1)
        rt = BrotliDecode(output, 2)
        assert rt == self.test_data
        assert len(output) < 1024 * 1024
    def test_memory_view(self):
        output = BrotliCompress(memoryview(self.test_data),
                                {
                                    BROTLI_PARAM_QUALITY:5,
                                    BROTLI_PARAM_CATABLE:1,
                                    BROTLI_PARAM_MAGIC_NUMBER:1,
                                },
                                8)
        rt = BrotliDecode(output)
        assert rt == self.test_data
        assert len(output) < 1024 * 1024
    def test_1(self):
        output = BrotliCompress(self.test_data[:65536],
                                {
                                    BROTLI_PARAM_QUALITY:11,
                                    BROTLI_PARAM_CATABLE:1,
                                    BROTLI_PARAM_MAGIC_NUMBER:1,
                                },
                                8)
        rt = BrotliDecode(output)
        assert rt == self.test_data[:65536]
        assert len(output) < 1024 * 1024
    def test_rnd(self):
        random_data = os.urandom(131072)
        wp = BrotliEncoderCreateWorkPool(8)
        output = BrotliEncoderCompressWorkPool(wp,
                                               random_data,
                                               {
                                                BROTLI_PARAM_QUALITY:7,
                                                   BROTLI_PARAM_CATABLE:1,
                                                   BROTLI_PARAM_MAGIC_NUMBER:1,
                                               },
                                               8)
        BrotliEncoderDestroyWorkPool(wp)
        rt = BrotliDecode(output)
        assert rt == random_data
        assert len(output) > 130000
    def test_truncation(self):
        output = BrotliCompress(self.test_data[:65536],
                                {
                                    BROTLI_PARAM_QUALITY:6,
                                    BROTLI_PARAM_CATABLE:1,
                                    BROTLI_PARAM_MAGIC_NUMBER:1,
                                },
                                8)
        corrupt = output[:len(output) - 1]
        rt = BrotliDecode(output)
        assert rt == self.test_data[:65536]
        assert len(output) < 1024 * 1024
        try:
            BrotliDecode(corrupt)
        except BrotliDecompressorException:
            pass
        else:
            assert False, "Should have errored"
    def test_corruption(self):
        output = BrotliCompress(self.test_data[:65536],
                                {
                                    BROTLI_PARAM_QUALITY:6,
                                    BROTLI_PARAM_CATABLE:1,
                                    BROTLI_PARAM_MAGIC_NUMBER:1,
                                },
                                8)
        corrupt = output[:len(output)/2] + output[len(output)/2 + 1:]
        rt = BrotliDecode(output)
        assert rt == self.test_data[:65536]
        assert len(output) < 1024 * 1024
        try:
            BrotliDecode(corrupt)
        except BrotliDecompressorException:
            pass
        else:
            assert False, "Should have errored"
if __name__ == '__main__':
    unittest.main()

