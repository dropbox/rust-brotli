#ifndef BROTLI_BROCCOLI_H
#define BROTLI_BROCCOLI_H
struct BroccoliState {
    void *unused;
    unsigned char data[248];
};

enum BroccoliResult {
  BroccoliSuccess = 0,
  BroccoliNeedsMoreInput = 1,
  BroccoliNeedsMoreOutput = 2,
  BroccoliBrotliFileNotCraftedForAppend = 124,
  BroccoliInvalidWindowSize = 125,
  BroccoliWindowSizeLargerThanPreviousFile = 126,
  BroccoliBrotliFileNotCraftedForConcatenation = 127,
};


struct BroccoliState BroccoliCreateInstance();

struct BroccoliState BroccoliCreateInstanceWithWindowSize(unsigned char window_size);

void BroccoliDestroyInstance(struct BroccoliState state);

void BroccoliNewBrotliFile(struct BroccoliState *state);

enum BroccoliResult BroccoliConcatStream(
    struct BroccoliState *state,
    size_t *available_in,
    const unsigned char **input_buf_ptr,
    size_t *available_out,
    unsigned char **output_buf_ptr);

enum BroccoliResult BroccoliConcatFinish(struct BroccoliState * state,
                              size_t *available_out,
                              unsigned char**output_buf);
#endif
