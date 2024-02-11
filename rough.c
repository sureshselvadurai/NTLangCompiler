#include <stdio.h>
#include <stdint.h>

void bitsToNBits(uint32_t *value, size_t n) {
    // Print original value
    printf("Original value: %u\n", *value);

    // Create a bitmask to isolate the least significant n bits
    uint32_t bitmask = (1 << n) - 1;

    // Extract the least significant n bits
    *value &= bitmask;
    printf("Original bitmask: %u\n", bitmask);

    // Print the modified value
    printf("Modified value: %u\n", *value);
}

int main() {
    uint32_t value = 0b10; // Example value
    bitsToNBits(&value, 31); // Extract least significant 4 bits
    return 0;
}
