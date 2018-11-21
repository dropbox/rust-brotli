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
    def test_1(self):
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
if __name__ == '__main__':
    unittest.main()

