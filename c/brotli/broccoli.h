#ifndef BROTLI_BROCCOLI_H
#define BROTLI_BROCCOLI_H
#include <stdint.h>
typedef struct BroccoliState_ {
    void *unused;
    unsigned char data[248];
} BroccoliState;

typedef enum BroccoliResult_ {
  BroccoliSuccess = 0,
  BroccoliNeedsMoreInput = 1,
  BroccoliNeedsMoreOutput = 2,
  BroccoliBrotliFileNotCraftedForAppend = 124,
  BroccoliInvalidWindowSize = 125,
  BroccoliWindowSizeLargerThanPreviousFile = 126,
  BroccoliBrotliFileNotCraftedForConcatenation = 127,
} BroccoliResult;


BroccoliState BroccoliCreateInstance();

BroccoliState BroccoliCreateInstanceWithWindowSize(uint8_t window_size);

void BroccoliDestroyInstance(BroccoliState state);

void BroccoliNewBrotliFile(BroccoliState *state);

BroccoliResult BroccoliConcatStream(
    BroccoliState *state,
    size_t *available_in,
    const uint8_t **input_buf_ptr,
    size_t *available_out,
    uint8_t **output_buf_ptr);

BroccoliResult BroccoliConcatStreaming(
    BroccoliState *state,
    size_t *available_in,
    const uint8_t *input_buf_ptr,
    size_t *available_out,
    uint8_t *output_buf_ptr);

BroccoliResult BroccoliConcatFinish(BroccoliState * state,
                              size_t *available_out,
                              uint8_t**output_buf);
BroccoliResult BroccoliConcatFinished(BroccoliState * state,
                              size_t *available_out,
                              uint8_t*output_buf);
#endif
