#include <stdio.h>
#include <stdlib.h>
#include "ntlang.h" // Assuming this header contains necessary declarations

void eval_error(char *err) {
    printf("eval_error: %s\n", err);
    exit(-1);
}

uint32_t eval(struct parse_node_st *pt) {
    uint32_t v1, v2;

    if (pt->type == EX_LITERAL) {
        return pt->literal.value;
    } else if (pt->type == EX_OPER1) {
        v1 = eval(pt->oper1.operand);
        switch (pt->oper1.oper) {
            case OP_PLUS:
                return v1;
            case OP_MINUS:
                return -v1;
            case OP_BIT_NOT:
                return ~v1;
            default:
                eval_error("Invalid unary operator");
        }
    } else if (pt->type == EX_OPER2) {
        v1 = eval(pt->oper2.left);
        v2 = eval(pt->oper2.right);
        switch (pt->oper2.oper) {
            case OP_PLUS:
                return v1 + v2;
            case OP_MINUS:
                return v1 - v2;
            case OP_MULT:
                return v1 * v2;
            case OP_DIV:
                if (v2 == 0) eval_error("Division by zero");
                return v1 / v2;
            case OP_SHIFT_RIGHT:
                return v1 >> v2;
            case OP_SHIFT_LEFT:
                return v1 << v2;
            case OP_ARITH_SHIFT_RIGHT:
//                Checks significant
                if (v1 & 0x80000000) {
                    for (int i = 0; i < v2; i++) {
//                    shifts by 1
                        v1 >>= 1;
//                    retains the significant
                        v1 |= 0x80000000;
                    }
                } else {
                    v1 >>= v2;
                }
                return v1;
            case OP_BIT_AND:
                return v1 & v2;
            case OP_BIT_OR:
                return v1 | v2;
            case OP_BIT_XOR:
                return v1 ^ v2;
            default:
                eval_error("Invalid binary operator");
        }
    }
    // Default return, in case something goes wrong
    return 0;
}

void eval_print(struct config_st *cp, uint32_t value) {
    /*
     * Handle -b -w -u
     *
     * Use your own conversion functions for uint32_t to string.
     */
    char str[SCAN_INPUT_LEN];
    int i=0;

    uint32_t n_bit_value = mask_value(value, cp->width);
    int sign = is_negative(n_bit_value, cp->width, cp->unsigned_int);

    switch (cp->base) {
        case 10:
            convert_to_decimal(n_bit_value, str, &i, sign,  cp->width);
            break;
        case 2:
            convert_to_binary(value, str, &i, cp->width);
            break;
        case 16:
            convert_to_hexadecimal(value, str, &i, cp->width);
            break;
    }


    int len = i;
    for (int j = 0; j < len / 2; j++) {
        char temp = str[j];
        str[j] = str[len - j - 1];
        str[len - j - 1] = temp;
    }

    str[i] = '\0';

    switch (cp->base) {
        case 2:
            printf("0b%s\n", str);
            break;
        case 10:
            printf("%s\n", str);
            break;
        case 16:
            printf("0x%s\n", str);
            break;
    }
}

uint32_t mask_value(uint32_t value, uint32_t width) {
//    to avoid overflow
    if(width!=32){
        value =  (value & ((1 << width) - 1));
    }
    return value;
}

bool is_negative(uint32_t n_bit_value, int width, int unsigned_int) {
    bool sign = false;
//    checking the significant digit
    if(unsigned_int){
        if (n_bit_value & (1 << (width - 1))) {
            return true;
        }
    }
    return sign;
}

void convert_to_decimal(int n_bit_value, char *str, int *i, int sign, int width) {
    if (sign) {
//        convert to positive
//        to avoid overflow
        if (width != 32) {
            int mask = (1 << width) - 1;
            n_bit_value ^= mask;
            n_bit_value += 1;
        } else {
            n_bit_value = ~n_bit_value + 1;
        }
    }
//    to avoid signage
    if (n_bit_value == 0) {
        str[(*i)++] = '0';
        return;
    }
    while (n_bit_value != 0) {
        int remainder = n_bit_value % 10;
        str[(*i)++] = remainder + '0';
        n_bit_value /= 10;
    }
    if (sign) {
        str[(*i)++] = '-';
    }
}

void convert_to_binary(uint32_t value, char *str, int *i, int width) {

    for (int j = 0; j < width ; j++) {
        uint32_t mask = 1 << j;
        char bit = (value & mask) ? '1' : '0';

        str[*i] = bit;
        (*i)++;
    }
}

void convert_to_hexadecimal(uint32_t value, char *str, int *i, int width) {
    if(width != 32) {
        uint32_t mask = ((uint32_t)1 << width) - 1;
        value &= mask;
    }

    for(int j = 0; j <= width - 4; j += 4) {
        uint32_t mask = 0xF << j;
        uint8_t hex_digit = (value & mask) >> j;

        char digit_char;
        if(hex_digit < 10) {
            digit_char = hex_digit + '0';
        } else {
            digit_char = hex_digit - 10 + 'A';
        }
        str[*i] = digit_char;
        (*i)++;
    }
}