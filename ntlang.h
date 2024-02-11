/* ntlang.h - header file for project01 (ntlang) */

#include <stdbool.h> 
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#define SCAN_TOKEN_LEN 32
#define SCAN_TABLE_LEN 1024
#define SCAN_INPUT_LEN 4096
#define PARSE_TABLE_LEN 1024

/*
 * scan.c
*/

/*
# Scanner EBNF (microsyntax)

tokenlist  ::= (token)*
token      ::= intlit | hexlit | binlit | symbol
symbol     ::= '+' | '-' | '*' | '/' | '>>' | '>-' | '<<' | '~' | '&' | '|' | '^'
intlit     ::= digit (digit)*
hexlit     ::= '0x' hexdigit (hexdigit)*
binlit     ::= '0b' ['0', '1'] (['0', '1'])*
hexdigit   ::= 'a' | ... | 'f' | 'A' | ... | 'F' | digit
digit      ::= '0' | ... | '9'

# Ignore
whitespace ::= ' ' | '\t' (' ' | '\t')*

*/

enum scan_token_enum {
    TK_INTLIT, /* 1, 22, 403 */
    TK_HEXLIT, /* 0x1A, 0xFF */
    TK_BINLIT, /* 0b1010, 0b1111 */
    TK_PLUS,   /* + */
    TK_MINUS,  /* - */
    TK_MULT,   /* * */
    TK_DIV,    /* / */
    TK_SHIFT_RIGHT, /* >> */
    TK_SHIFT_LEFT,  /* << */
    TK_ARITH_SHIFT_RIGHT, /* >- */
    TK_BIT_AND, /* & */
    TK_BIT_OR,  /* | */
    TK_BIT_XOR, /* ^ */
    TK_BIT_NOT, /* ~ */
    TK_LPAREN, /* ( */
    TK_RPAREN, /* ) */
    TK_EOT,    /* end of text */
    TK_ANY,    /* A wildcard token*/
};

#define SCAN_TOKEN_STRINGS {\
    "TK_INTLIT",\
    "TK_HEXLIT",\
    "TK_BINLIT",\
    "TK_PLUS",\
    "TK_MINUS",\
    "TK_MULT",\
    "TK_DIV",\
    "TK_SHIFT_RIGHT",\
    "TK_SHIFT_LEFT",\
    "TK_ARITH_SHIFT_RIGHT",\
    "TK_BIT_AND",\
    "TK_BIT_OR",\
    "TK_BIT_XOR",\
    "TK_BIT_NOT",\
    "TK_LPAREN",\
    "TK_RPAREN",\
    "TK_EOT",\
    "TK_ANY"\
};

struct scan_token_st {
    enum scan_token_enum id;
    char value[SCAN_TOKEN_LEN];
};

struct scan_table_st {
    struct scan_token_st table[SCAN_TABLE_LEN];
    int len;
    int cur;
};

void scan_table_init(struct scan_table_st *st);
void scan_table_scan(struct scan_table_st *st, char *input);
void scan_table_print(struct scan_table_st *st);
struct scan_token_st * scan_table_get(struct scan_table_st *st, int i);
bool scan_table_accept(struct scan_table_st *st, enum scan_token_enum tk_expected);


/*
 * parse.c
*/

/*
A simple grammar for the ntcalc langauge

# Parser

program    ::= expression EOT
expression ::= operand (operator operand)*
operand    ::= intlit
             | hexlit
             | binlit
             | '-' operand
             | '~' operand
             | '(' expression ')'
operator   ::= '+' | '-' | '*' | '/' | '>>' | '<<' | '&' | '|' | '^' | '>-' | '~'

*/

enum parse_expr_enum {EX_LITERAL, EX_OPER1, EX_OPER2};
enum parse_oper_enum {OP_PLUS, OP_MINUS, OP_MULT, OP_DIV, OP_SHIFT_RIGHT, OP_SHIFT_LEFT, OP_ARITH_SHIFT_RIGHT, OP_BIT_AND, OP_BIT_OR, OP_BIT_XOR, OP_BIT_NOT};

struct parse_node_st {
    enum parse_expr_enum type;
    union {
        struct { uint32_t value; } literal;
        struct { enum parse_oper_enum oper; struct parse_node_st *operand; } oper1;
        struct { enum parse_oper_enum oper; struct parse_node_st *left; struct parse_node_st *right; } oper2;
    };
};

struct parse_table_st {
    struct parse_node_st table[PARSE_TABLE_LEN];
    int len;
};



void parse_table_init(struct parse_table_st *pt);
struct parse_node_st *parse_node_new(struct parse_table_st *pt);
struct parse_node_st *parse_program(struct parse_table_st *pt, struct scan_table_st *st);
struct parse_node_st *parse_expression(struct parse_table_st *pt, struct scan_table_st *st);
struct parse_node_st *parse_operand(struct parse_table_st *pt, struct scan_table_st *st);
struct parse_node_st *parse_literal(struct parse_table_st *pt, struct scan_table_st *st);
void parse_tree_print(struct parse_node_st *np);
void parse_error(const char *err);

/*
 * config
 */

struct config_st {
    char input[SCAN_INPUT_LEN];
    int base;
    int width;
    bool unsigned_int;
};

void parse_args(struct config_st *cp, int argc, char **argv);

/*
 * eval.c
 */

uint32_t eval(struct parse_node_st *pt);
void eval_print(struct config_st *cp, uint32_t value);
void int_to_str(uint32_t value, char *str, int base, int width,struct config_st *cp);
