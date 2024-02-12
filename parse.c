/* parse.c - parsing and parse tree construction */

#include "ntlang.h"

void parse_table_init(struct parse_table_st *pt) {
    pt->len = 0;
}

struct parse_node_st * parse_node_new(struct parse_table_st *pt) {
    struct parse_node_st *np;

    np = &(pt->table[pt->len]);
    pt->len += 1;

    return np;
}

void parse_error(const char *err) {
    printf("%s\n", err);
    exit(-1);
}

void parse_error_message(const char *err, const char *message) {
    printf("%s%s\n", err, message);
    exit(-1);
}

char *parse_oper_strings[] = {"PLUS", "MINUS", "MULT", "DIV", "SHIFT_RIGHT", "SHIFT_LEFT", "ARITH_SHIFT_RIGHT", "BIT_AND", "BIT_OR", "BIT_XOR", "BIT_NOT"};

/* We need to provide prototypes for the parsing functions because
 * we call them before they are defined.
 */
struct parse_node_st * parse_program(struct parse_table_st *pt, 
                                        struct scan_table_st *st);
struct parse_node_st * parse_expression(struct parse_table_st *pt, 
                                        struct scan_table_st *st);
struct parse_node_st * parse_operand(struct parse_table_st *pt, 
                                        struct scan_table_st *st);

/* We need a parsing function for each rule in the EBNF grammer */

struct parse_node_st * parse_program(struct parse_table_st *pt, 
                                        struct scan_table_st *st) {
    struct parse_node_st *np1;

    /* A program is a single expression followed by EOT */
    np1 = parse_expression(pt, st);

    if (!scan_table_accept(st, TK_EOT)) {
        parse_error("Expecting EOT");
    }

    return np1;
}

struct parse_node_st * parse_expression(struct parse_table_st *pt, 
                                        struct scan_table_st *st) {
    struct scan_token_st *tp;
    struct parse_node_st *np1, *np2;

    /* An expression must start with an operand. */
    np1 = parse_operand(pt, st);

    while (true) {
        tp = scan_table_get(st, 0);
        /* Check for valid operator */
        if (tp->id == TK_PLUS || tp->id == TK_MINUS || tp->id == TK_MULT || tp->id == TK_DIV || tp->id == TK_SHIFT_RIGHT || tp->id == TK_SHIFT_LEFT || tp->id == TK_ARITH_SHIFT_RIGHT || tp->id == TK_BIT_AND || tp->id == TK_BIT_OR || tp->id == TK_BIT_XOR) {
            scan_table_accept(st, TK_ANY);
            np2 = parse_node_new(pt);
            np2->type = EX_OPER2;
            np2->oper2.oper = tp->id - TK_PLUS;
            np2->oper2.left = np1;
            /* parse second operand */
            np2->oper2.right = parse_operand(pt, st);
            np1 = np2;
        } else {
            break;
        }
    }
    return np1;
}

struct parse_node_st * parse_operand(struct parse_table_st *pt,
                                     struct scan_table_st *st) {
    struct scan_token_st *tp;
    struct parse_node_st *np1;

    if (scan_table_accept(st, TK_INTLIT)) {
            tp = scan_table_get(st, -1);
            np1 = parse_node_new(pt);
            np1->type = EX_LITERAL;
            np1->literal.value = 0;
            for (int i = 0; tp->value[i] != '\0'; i++) {
                if (tp->value[i] < '0' || tp->value[i] > '9') {
                    parse_error("Invalid integer literal");
                }
                np1->literal.value = np1->literal.value * 10 + (tp->value[i] - '0');
            }
        }else if (scan_table_accept(st, TK_HEXLIT)) {
             tp = scan_table_get(st, -1);
             np1 = parse_node_new(pt);
             np1->type = EX_LITERAL;
             np1->literal.value = 0;
             for (int i = 0; tp->value[i] != '\0'; i++) {
                 if (i >= 0) {
                     if (tp->value[i] >= '0' && tp->value[i] <= '9') {
                         // Check for overflow before performing addition
                         if (np1->literal.value <= (UINT32_MAX - (tp->value[i] - '0')) / 16) {
                             np1->literal.value = np1->literal.value * 16 + (tp->value[i] - '0');
                         } else {
                             parse_error_message("overflows uint32_t: ", tp->value);
                         }
                     } else if (tp->value[i] >= 'A' && tp->value[i] <= 'F') {
                         // Check for overflow before performing addition
                         if (np1->literal.value <= (UINT32_MAX - (tp->value[i] - 'A' + 10)) / 16) {
                             np1->literal.value = np1->literal.value * 16 + (tp->value[i] - 'A' + 10);
                         } else {
                             parse_error_message("overflows uint32_t: ", tp->value);
                         }
                     } else if (tp->value[i] >= 'a' && tp->value[i] <= 'f') {
                         // Check for overflow before performing addition
                         if (np1->literal.value <= (UINT32_MAX - (tp->value[i] - 'a' + 10)) / 16) {
                             np1->literal.value = np1->literal.value * 16 + (tp->value[i] - 'a' + 10);
                         } else {
                             parse_error_message("overflows uint32_t: ", tp->value);
                         }
                     } else {
                         parse_error("Invalid hexadecimal literal");
                     }
                 }
             }
         } else if (scan_table_accept(st, TK_BINLIT)) {
            tp = scan_table_get(st, -1);
            np1 = parse_node_new(pt);
            np1->type = EX_LITERAL;
            np1->literal.value = 0;
            for (int i = 0; tp->value[i] != '\0'; i++) {
                if (i >= 0) {
                    if (tp->value[i] == '0' || tp->value[i] == '1') {
                        np1->literal.value = np1->literal.value * 2 + (tp->value[i] - '0');
                    } else {
                        parse_error("Invalid binary literal");
                    }
                }
            }
        } else if (scan_table_accept(st, TK_MINUS)) {
            np1 = parse_node_new(pt);
            np1->type = EX_OPER1;
            np1->oper1.oper = OP_MINUS;
            np1->oper1.operand = parse_operand(pt, st);
        } else if (scan_table_accept(st, TK_BIT_NOT)) {
            np1 = parse_node_new(pt);
            np1->type = EX_OPER1;
            np1->oper1.oper = OP_BIT_NOT;
            np1->oper1.operand = parse_operand(pt, st);
        } else if (scan_table_accept(st, TK_LPAREN)) {
            np1 = parse_expression(pt, st);
            if (!scan_table_accept(st, TK_RPAREN)) {
                parse_error("Expecting ')'");
            }
        } else {
            parse_error("Bad operand");
        }
    return np1;
}

void parse_tree_print_indent(int level) {
    level *= 2;
    for (int i = 0; i < level; i++) {
        printf(".");
    }
}

void parse_tree_print_expr(struct parse_node_st *np, int level) {
    parse_tree_print_indent(level);
    printf("EXPR ");

    if (np->type == EX_LITERAL) {
        printf("LITERAL %d\n", np->literal.value);
    } else if (np->type == EX_OPER1) {
        printf("OPER1 %s\n", parse_oper_strings[np->oper1.oper]);
        parse_tree_print_expr(np->oper1.operand, level+1);
    } else if (np->type == EX_OPER2) {
        printf("OPER2 %s\n", parse_oper_strings[np->oper2.oper]);
        parse_tree_print_expr(np->oper2.left, level+1);
        parse_tree_print_expr(np->oper2.right, level+1);
    }
}

void parse_tree_print(struct parse_node_st *np) {
    parse_tree_print_expr(np, 0);    
}
