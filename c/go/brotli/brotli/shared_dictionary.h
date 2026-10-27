/* Copyright 2017 Google Inc. All Rights Reserved.

   Distributed under MIT license.
   See file LICENSE for detail or copy at https://opensource.org/licenses/MIT
*/

/* Shared Dictionary public types used by the decoder API. */

#ifndef BROTLI_COMMON_SHARED_DICTIONARY_H_
#define BROTLI_COMMON_SHARED_DICTIONARY_H_

#include "port.h"
#include "types.h"

#if defined(__cplusplus) || defined(c_plusplus)
extern "C" {
#endif

/**
 * Input data type for ::BrotliDecoderAttachDictionary.
 */
typedef enum BrotliSharedDictionaryType {
  /** Raw LZ77 prefix dictionary. */
  BROTLI_SHARED_DICTIONARY_RAW = 0,
  /** Serialized shared dictionary in the 0x91 0x00 shared-brotli container. */
  BROTLI_SHARED_DICTIONARY_SERIALIZED = 1
} BrotliSharedDictionaryType;

#if defined(__cplusplus) || defined(c_plusplus)
}  /* extern "C" */
#endif

#endif  /* BROTLI_COMMON_SHARED_DICTIONARY_H_ */
