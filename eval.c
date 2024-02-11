#include <stdio.h>
#include <stdlib.h>
#include "ntlang.h" // Assuming this header contains necessary declarations

void eval_error(char *err);
uint32_t eval(struct parse_node_st *pt);
void eval_print(struct config_st *cp, uint32_t value);
void int_to_str(uint32_t value, char *str, int base, int width,struct config_st *cp);
void convert_to_binary(uint32_t value, char *str, int *i, int width);
void convert_to_decimal(int32_t value, char *str, int *i, int width);
void convert_to_positive_decimal(uint32_t value, char *str, int *i, int width);
void convert_to_hexadecimal(uint32_t value, char *str, int *i, int width);
void bitsToNBits(uint32_t *value, size_t n);

void decimalToBinary(uint32_t decimal) {
    uint32_t binary[32];
    int i = 0;
    while (decimal > 0) {
        binary[i] = decimal % 2;
        decimal = decimal / 2;
        i++;
    }

    printf("Binary representation: ");
    for (int j = i - 1; j >= 0; j--) {
        printf("%u", binary[j]);
    }
    printf("\n");
}

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
                if (v1 & 0x80000000) {
                    for (int i = 0; i < v2; i++) {
                        v1 >>= 1;
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

    int_to_str(value, str, cp->base, cp->width, cp);

    if (cp->unsigned_int && cp->base == 10) {
        printf("%u\n", value);
    } else {
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
}

void int_to_str(uint32_t value, char *str, int base, int width, struct config_st *cp) {
    int i = 0;
    int remainder;
    int quotient = value;

    if (value < 0) {
        str[i++] = '-';
        value = (~value) + 1;
    }

    switch (base) {
        case 2:
            convert_to_binary(value, str, &i, width);
            break;
        case 10:
            if(cp->unsigned_int){
                convert_to_positive_decimal(value, str, &i, width);
                }
            else{
                convert_to_decimal(value, str, &i, width);
            }


            break;
        case 16:
            convert_to_hexadecimal(value, str, &i, width);
            break;
    }

    int len = i;
    for (int j = 0; j < len / 2; j++) {
        char temp = str[j];
        str[j] = str[len - j - 1];
        str[len - j - 1] = temp;
    }
    str[i] = '\0';
}

void convert_to_binary(uint32_t value, char *str, int *i, int width) {
    if(width!=32){
        uint32_t mask = ((uint32_t)1 << width) - 1;
        value &= mask;
    }

    for (int j = 0; j < width ; j++) {
        uint32_t mask = 1 << j;
        char bit = (value & mask) ? '1' : '0';

        str[*i] = bit;
        (*i)++;
    }
}

void convert_to_decimal(int32_t value, char *str, int *i, int width) {
    // Mask to retain only the least significant 'width' bits

    if(width != 32) {
        uint32_t mask = ((uint32_t)1 << width) - 1;
        value &= mask;
    }

    // Convert negative value to positive for processing

    if (value < 0) {
        value = ~value+1;
    }

    // Convert to decimal
    if (value == 0) {
        str[(*i)++] = '0';
        return;
    }

    while (value != 0) {
        int remainder = value % 10;
        str[(*i)++] = remainder + '0';
        value /= 10;
    }
    str[(*i)++] = '-';

}

void convert_to_positive_decimal(uint32_t value, char *str, int *i, int width) {
    if(width != 32) {
        uint32_t mask = ((uint32_t)1 << width) - 1;
        value &= mask;
    }

    if (value == 0) {
        str[(*i)++] = '0';
        return;
    }

    while (value != 0) {
        int remainder = value % 10;
        str[(*i)++] = remainder + '0';
        value /= 10;
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