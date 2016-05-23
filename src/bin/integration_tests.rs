#![cfg(test)]
extern crate core;
use std::io;
use core::cmp;
struct Buffer {
    data : Vec<u8>,
    read_offset : usize,
}
impl Buffer {
    pub fn new(buf : &[u8]) -> Buffer {
        let mut ret = Buffer{
            data : Vec::<u8>::new(),
            read_offset : 0,
        };
        ret.data.extend(buf);
        return ret;
    }
}
impl io::Read for Buffer {
    fn read(self : &mut Self, buf : &mut [u8]) -> io::Result<usize> {
        let bytes_to_read = cmp::min(buf.len(), self.data.len() - self.read_offset);
        if bytes_to_read > 0 {
            buf[0..bytes_to_read].clone_from_slice(&self.data[self.read_offset ..
                                                                     self.read_offset + bytes_to_read]);
        }
        self.read_offset += bytes_to_read;
        return Ok(bytes_to_read);
    }
}
impl io::Write for Buffer {
    fn write(self : &mut Self, buf : &[u8]) -> io::Result<usize> {
        self.data.extend(buf);
        return Ok(buf.len());
    }
    fn flush(self : &mut Self) -> io::Result<()> {
        return Ok(());
    }
}
#[test]
fn test_10x_10y() {
    let in_buf : [u8;12] = [0x1b, 0x13, 0x00, 0x00, 0xa4, 0xb0, 0xb2, 0xea, 0x81, 0x47, 0x02, 0x8a];
    let mut input = Buffer::new(&in_buf);
    let mut output = Buffer::new(&[]);
    match super::decompress(&mut input, &mut output) {
        Ok(_) => {},
        Err(e) => panic!("Error {:?}", e),
    }
    let mut i : usize = 0;
    while i < 10 {
      assert_eq!(output.data[i], 'X' as u8);
      assert_eq!(output.data[i + 10], 'Y' as u8);
      i += 1;
    }
    assert_eq!(output.data.len(), 20);
    assert_eq!(input.read_offset, in_buf.len());
}

#[test]
fn test_10x_10y_one_out_byte() {
    let in_buf : [u8;12] = [0x1b, 0x13, 0x00, 0x00, 0xa4, 0xb0, 0xb2, 0xea, 0x81, 0x47, 0x02, 0x8a];
    let mut input = Buffer::new(&in_buf);
    let mut output = Buffer::new(&[]);
    match super::decompress_internal(&mut input, &mut output, 12, 1) {
        Ok(_) => {},
        Err(e) => panic!("Error {:?}", e),
    }
    let mut i : usize = 0;
    while i < 10 {
      assert_eq!(output.data[i], 'X' as u8);
      assert_eq!(output.data[i + 10], 'Y' as u8);
      i += 1;
    }
    assert_eq!(output.data.len(), 20);
    assert_eq!(input.read_offset, in_buf.len());
}

#[test]
fn test_10x_10y_byte_by_byte() {
    let in_buf : [u8;12] = [0x1b, 0x13, 0x00, 0x00, 0xa4, 0xb0, 0xb2, 0xea, 0x81, 0x47, 0x02, 0x8a];
    let mut input = Buffer::new(&in_buf);
    let mut output = Buffer::new(&[]);
    match super::decompress_internal(&mut input, &mut output, 1, 1) {
        Ok(_) => {},
        Err(e) => panic!("Error {:?}", e),
    }
    let mut i : usize = 0;
    while i < 10 {
      assert_eq!(output.data[i], 'X' as u8);
      assert_eq!(output.data[i + 10], 'Y' as u8);
      i += 1;
    }
    assert_eq!(output.data.len(), 20);
    assert_eq!(input.read_offset, in_buf.len());
}


fn assert_decompressed_input_matches_output(input_slice : &[u8],
                                            output_slice : &[u8],
                                            input_buffer_size : usize,
                                            output_buffer_size : usize) {
    let mut input = Buffer::new(input_slice);
    let mut output = Buffer::new(&[]);
    match super::decompress_internal(&mut input, &mut output, input_buffer_size, output_buffer_size) {
       Ok(_) => {}
       Err(e) => panic!("Error {:?}", e),
    }
    assert_eq!(output.data.len(), output_slice.len());
    assert_eq!(output.data, output_slice)
}

#[test]
fn test_64x() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/64x.compressed"),
                                             include_bytes!("testdata/64x"),
                                             3,
                                             3);
}

#[test]
fn test_as_you_like_it() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/asyoulik.txt.compressed"),
                                             include_bytes!("testdata/asyoulik.txt"),
                                             65536,
                                             65536);
}


#[test]
#[should_panic]
fn test_negative_hypothesis() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/64x"),
                                             include_bytes!("testdata/64x"),
                                             3,
                                             3);
}

#[test]
fn test_alice29() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/alice29.txt.compressed"),
                                             include_bytes!("testdata/alice29.txt"),
                                             65536,
                                             65536);
}

#[test]
fn test_alice1() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/alice29.txt.compressed"),
                                             include_bytes!("testdata/alice29.txt"),
                                             1,
                                             65536);
}

#[test]
fn test_backward65536() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/backward65536.compressed"),
                                             include_bytes!("testdata/backward65536"),
                                             65536,
                                             65536);
}


#[test]
fn test_compressed_file() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/compressed_file.compressed"),
                                             include_bytes!("testdata/compressed_file"),
                                             65536,
                                             65536);
}

#[test]
fn test_compressed_repeated() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/compressed_repeated.compressed"),
                                             include_bytes!("testdata/compressed_repeated"),
                                             65536,
                                             65536);
}

#[test]
fn test_empty() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/empty.compressed"),
                                             include_bytes!("testdata/empty"),
                                             65536,
                                             65536);
}
#[test]
fn test_empty0() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/empty.compressed.00"),
                                             include_bytes!("testdata/empty"),
                                             65536,
                                             65536);
}
#[test]
fn test_empty1() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/empty.compressed.01"),
                                             include_bytes!("testdata/empty"),
                                             65536,
                                             65536);
}
#[test]
fn test_empty2() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/empty.compressed.02"),
                                             include_bytes!("testdata/empty"),
                                             65536,
                                             65536);
}
#[test]
fn test_empty3() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/empty.compressed.03"),
                                             include_bytes!("testdata/empty"),
                                             65536,
                                             65536);
}
#[test]
fn test_empty4() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/empty.compressed.04"),
                                             include_bytes!("testdata/empty"),
                                             65536,
                                             65536);
}
#[test]
fn test_empty5() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/empty.compressed.05"),
                                             include_bytes!("testdata/empty"),
                                             65536,
                                             65536);
}
#[test]
fn test_empty6() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/empty.compressed.06"),
                                             include_bytes!("testdata/empty"),
                                             65536,
                                             65536);
}
#[test]
fn test_empty7() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/empty.compressed.07"),
                                             include_bytes!("testdata/empty"),
                                             65536,
                                             65536);
}
#[test]
fn test_empty8() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/empty.compressed.08"),
                                             include_bytes!("testdata/empty"),
                                             65536,
                                             65536);
}
#[test]
fn test_empty9() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/empty.compressed.09"),
                                             include_bytes!("testdata/empty"),
                                             65536,
                                             65536);
}
#[test]
fn test_empty10() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/empty.compressed.10"),
                                             include_bytes!("testdata/empty"),
                                             65536,
                                             65536);
}
#[test]
fn test_empty11() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/empty.compressed.11"),
                                             include_bytes!("testdata/empty"),
                                             65536,
                                             65536);
}
#[test]
fn test_empty12() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/empty.compressed.12"),
                                             include_bytes!("testdata/empty"),
                                             65536,
                                             65536);
}
#[test]
fn test_empty13() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/empty.compressed.13"),
                                             include_bytes!("testdata/empty"),
                                             65536,
                                             65536);
}
#[test]
fn test_empty14() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/empty.compressed.14"),
                                             include_bytes!("testdata/empty"),
                                             65536,
                                             65536);
}
#[test]
fn test_empty15() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/empty.compressed.15"),
                                             include_bytes!("testdata/empty"),
                                             65536,
                                             65536);
}
#[test]
fn test_empty16() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/empty.compressed.16"),
                                             include_bytes!("testdata/empty"),
                                             65536,
                                             65536);
}
#[test]
fn test_empty17() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/empty.compressed.17"),
                                             include_bytes!("testdata/empty"),
                                             65536,
                                             65536);
}
#[test]
fn test_empty18() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/empty.compressed.18"),
                                             include_bytes!("testdata/empty"),
                                             65536,
                                             65536);
}

#[test]
fn lcet10() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/lcet10.txt.compressed"),
                                             include_bytes!("testdata/lcet10.txt"),
                                             65536,
                                             65536);
}

#[test]
fn test_mapsdatazrh() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/mapsdatazrh.compressed"),
                                             include_bytes!("testdata/mapsdatazrh"),
                                             65536,
                                             65536);
}

#[test]
fn test_monkey() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/monkey.compressed"),
                                             include_bytes!("testdata/monkey"),
                                             65536,
                                             65536);
}

#[test]
fn test_monkey1() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/monkey.compressed"),
                                             include_bytes!("testdata/monkey"),
                                             1,
                                             1);
}

#[test]
fn test_monkey3() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/monkey.compressed"),
                                             include_bytes!("testdata/monkey"),
                                             3,
                                             65536);
}

#[test]
fn test_plrabn12() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/plrabn12.txt.compressed"),
                                             include_bytes!("testdata/plrabn12.txt"),
                                             65536,
                                             65536);
}

#[test]
fn test_random_org_10k() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/random_org_10k.bin.compressed"),
                                             include_bytes!("testdata/random_org_10k.bin"),
                                             65536,
                                             65536);
}

#[test]
fn test_ukkonooa() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/ukkonooa.compressed"),
                                             include_bytes!("testdata/ukkonooa"),
                                             65536,
                                             65536);
}

#[test]
fn test_ukkonooa3() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/ukkonooa.compressed"),
                                             include_bytes!("testdata/ukkonooa"),
                                             3,
                                             3);
}

#[test]
fn test_ukkonooa1() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/ukkonooa.compressed"),
                                             include_bytes!("testdata/ukkonooa"),
                                             1,
                                             1);
}

#[test]
fn test_x() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/x.compressed"),
                                             include_bytes!("testdata/x"),
                                             65536,
                                             65536);
}
#[test]
fn test_x_0() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/x.compressed.00"),
                                             include_bytes!("testdata/x"),
                                             65536,
                                             65536);
}
#[test]
fn test_x_1() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/x.compressed.01"),
                                             include_bytes!("testdata/x"),
                                             65536,
                                             65536);
}
#[test]
fn test_x_2() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/x.compressed.02"),
                                             include_bytes!("testdata/x"),
                                             65536,
                                             65536);
}
#[test]
fn test_x_3() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/x.compressed.03"),
                                             include_bytes!("testdata/x"),
                                             65536,
                                             65536);
}

#[test]
fn test_xyzzy() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/xyzzy.compressed"),
                                             include_bytes!("testdata/xyzzy"),
                                             65536,
                                             65536);
}

#[test]
fn test_zeros() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/zeros.compressed"),
                                             include_bytes!("testdata/zeros"),
                                             65536,
                                             65536);
}


#[test]
fn test_metablock_reset() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/metablock_reset.compressed"),
                                             include_bytes!("testdata/metablock_reset"),
                                             65536,
                                             65536);
}
/*
#[test]
fn test_metablock_reset1_65536() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/metablock_reset.compressed"),
                                             include_bytes!("testdata/metablock_reset"),
                                             1,
                                             65536);
}

#[test]
fn test_metablock_reset65536_1() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/metablock_reset.compressed"),
                                             include_bytes!("testdata/metablock_reset"),
                                             65536,
                                             1);
}

#[test]
fn test_metablock_reset1() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/metablock_reset.compressed"),
                                             include_bytes!("testdata/metablock_reset"),
                                             1,
                                             1);
}
*/
#[test]
fn test_metablock_reset3() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/metablock_reset.compressed"),
                                             include_bytes!("testdata/metablock_reset"),
                                             3,
                                             3);
}

#[test]
#[should_panic]
fn test_broken_file() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/borked.compressed"),
                                             include_bytes!("testdata/empty"),
                                             65536,
                                             65536);
}

#[test]
fn test_ends_with_truncated_dictionary() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/ends_with_truncated_dictionary.compressed"),
                                             include_bytes!("testdata/ends_with_truncated_dictionary"),
                                             65536,
                                             65536);
}

#[test]
fn test_random_then_unicode() {
    assert_decompressed_input_matches_output(include_bytes!("testdata/random_then_unicode.compressed"),
                                             include_bytes!("testdata/random_then_unicode"),
                                             65536,
                                             65536);
}

