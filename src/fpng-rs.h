#include "fpng.h"

std::vector<uint8_t> encode_buffer(uint8_t *image_start, uint32_t width,
                                   uint32_t height, uint32_t channels,
                                   bool *failed);

void init_fpng();