#include "fpng-rs.h"

#include <chrono>
#include <iostream>
#include <ostream>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <vector>

void init_fpng() { fpng::fpng_init(); }

std::vector<uint8_t> encode_buffer(uint8_t *image_start, uint32_t width,
                                   uint32_t height, uint32_t channels,
                                   bool *failed) {

    std::vector<uint8_t> fpng_file_buf;

    *failed = fpng::fpng_encode_image_to_memory(image_start, width, height,
                                                channels, fpng_file_buf);

    return fpng_file_buf;
}