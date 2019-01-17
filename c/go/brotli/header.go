package brotli

import (
	"encoding/hex"
	"errors"
)

func BrotliParseHeader(data []byte) (returnedVersion byte, returnedSize uint64, err error) {
	if len(data) < 4 {
		return 0, 0, errors.New("Insufficient data provided in string: " + hex.EncodeToString(data))
	}
	hdr := uint64(data[0]) + uint64(data[1])*256 + uint64(data[2])*65536 + uint64(data[3])*65536*256
	var bits uint
	if hdr&1 == 0 {
		bits = 1
	} else if (hdr & 15) != 1 {
		bits = 4
	} else if (hdr & 127) != 0x11 {
		bits = 7
	} else {
		bits = 14
	}
	hdr >>= bits
	if (hdr & 1) != 0 {
		return 0, 0, errors.New("Header incorrectly marked as last block")
	}
	hdr >>= 1
	bits += 1
	if (hdr & 3) != 3 {
		return 0, 0, errors.New("Header incorrectly contains file data")
	}
	hdr >>= 2
	bits += 2
	if (hdr & 1) != 0 {
		return 0, 0, errors.New("Reserved 0 metadata bit is set to nonzero value")
	}
	hdr >>= 1
	bits += 1
	if (hdr & 3) != 1 {
		return 0, 0, errors.New("Header should only need 1 byte of length data")
	}
	hdr >>= 2
	bits += 2
	num_raw_header_bytes := 1 + (hdr & 0xff)
	bits += 8
	byte_offset := ((uint64(bits) + 7) / 8)
	if uint64(len(data)) < byte_offset+num_raw_header_bytes {
		return 0, 0, errors.New("Insufficient data to accomodate number of raw header bytes " + hex.EncodeToString(data))
	}
	if data[byte_offset] != 0xe1 || data[byte_offset+1] != 0x97 || (data[byte_offset+2]&0xf0) != 0x80 || (data[byte_offset+2]&0xf) > 2 {
		return 0, 0, errors.New("Header does not start with E1978X: " + hex.EncodeToString(data[byte_offset:byte_offset+3]))
	}
	version := data[byte_offset+3]
	size_le := data[byte_offset+4 : byte_offset+num_raw_header_bytes]
	total_size := uint64(0)
	for index := uint(0); index < uint(len(size_le)); index += 1 {
		total_size += uint64(size_le[index]&0x7f) << (7 * index)
		if 0 == (size_le[index] & 0x80) {
			break
		}
	}
	return version, total_size, nil
}
