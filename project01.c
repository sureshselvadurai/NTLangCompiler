/* project01.c - NTlang - Number Tool Language */

#include "ntlang.h"

int main(int argc, char **argv) {

    struct config_st config;
    struct scan_table_st scan_table;
    struct parse_table_st parse_table;
    struct parse_node_st *parse_tree;
    uint32_t value;

    parse_args(&config, argc, argv);

    scan_table_init(&scan_table);
    scan_table_scan(&scan_table, config.input);
    
    parse_table_init(&parse_table);
    parse_tree = parse_program(&parse_table, &scan_table);

    value = eval(parse_tree);
    eval_print(&config, value);

    return 0;
}

void parse_args(struct config_st *cp, int argc, char **argv) {
    int i = 1;

    cp->base = 10;
    cp->width = 32;
    cp->unsigned_int = false;

    if (argc <= 2) {
        printf("Usage: project01 -e expression [-b base] [-w width] [-u]\n");
        printf("Example: project01 -e \"1 + 2\" -b 10 -w 32 -u\n");
        exit(-1);
    }

    while (i < argc) {
        if (strcmp(argv[i], "-e") == 0) {
            i += 1;
            strncpy(cp->input, argv[i], SCAN_INPUT_LEN);
        } else if (strcmp(argv[i], "-b") == 0) {
            i += 1;
            if (strcmp(argv[i], "2") == 0) {
                cp->base = 2;
            } else if (strcmp(argv[i], "10") == 0) {
                cp->base = 10;
            } else if (strcmp(argv[i], "16") == 0) {
                cp->base = 16;
            } else {
                printf("Invalid number base: %s\n", argv[i]);
                exit(-1);
            }
        } else if (strcmp(argv[i], "-w") == 0) {
            i += 1;
            if (strcmp(argv[i], "4") == 0) {
                cp->width = 4;
            } else if (strcmp(argv[i], "8") == 0) {
                cp->width = 8;
            } else if (strcmp(argv[i], "16") == 0) {
                cp->width = 16;
            } else if (strcmp(argv[i], "32") == 0) {
                cp->width = 32;
            } else {
                printf("Invalid width: %s\n", argv[i]);
                exit(-1);
            }
        } else if (strcmp(argv[i], "-u") == 0) {
            cp->unsigned_int = true;
        }
        i += 1;
    }

    if (strnlen(cp->input, SCAN_INPUT_LEN) == 0) {
        printf("No expression given to evaluate\n");
        exit(-1);
    }
}
