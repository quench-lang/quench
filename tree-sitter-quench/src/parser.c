#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 12
#define STATE_COUNT 52
#define LARGE_STATE_COUNT 26
#define SYMBOL_COUNT 37
#define ALIAS_COUNT 0
#define TOKEN_COUNT 20
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 12
#define MAX_ALIAS_SEQUENCE_LENGTH 4

enum {
  sym_comment = 1,
  anon_sym_COLON_EQ = 2,
  anon_sym_SEMI = 3,
  anon_sym_LPAREN = 4,
  anon_sym_RPAREN = 5,
  anon_sym_null = 6,
  anon_sym_true = 7,
  anon_sym_false = 8,
  sym_integer = 9,
  sym_string = 10,
  sym_symbol = 11,
  anon_sym_LBRACK = 12,
  anon_sym_COMMA = 13,
  anon_sym_RBRACK = 14,
  anon_sym_LBRACE = 15,
  anon_sym_RBRACE = 16,
  anon_sym_COLON = 17,
  sym_identifier = 18,
  anon_sym_EQ_GT = 19,
  sym_source_file = 20,
  sym_declaration = 21,
  sym_statement = 22,
  sym__expression = 23,
  sym_parenthesized = 24,
  sym__literal = 25,
  sym_boolean = 26,
  sym_list = 27,
  sym_map = 28,
  sym_entry = 29,
  sym_block = 30,
  sym_call = 31,
  sym_function = 32,
  aux_sym_source_file_repeat1 = 33,
  aux_sym_list_repeat1 = 34,
  aux_sym_map_repeat1 = 35,
  aux_sym_block_repeat1 = 36,
};

static const char *ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_comment] = "comment",
  [anon_sym_COLON_EQ] = ":=",
  [anon_sym_SEMI] = ";",
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [anon_sym_null] = "null",
  [anon_sym_true] = "true",
  [anon_sym_false] = "false",
  [sym_integer] = "integer",
  [sym_string] = "string",
  [sym_symbol] = "symbol",
  [anon_sym_LBRACK] = "[",
  [anon_sym_COMMA] = ",",
  [anon_sym_RBRACK] = "]",
  [anon_sym_LBRACE] = "{",
  [anon_sym_RBRACE] = "}",
  [anon_sym_COLON] = ":",
  [sym_identifier] = "identifier",
  [anon_sym_EQ_GT] = "=>",
  [sym_source_file] = "source_file",
  [sym_declaration] = "declaration",
  [sym_statement] = "statement",
  [sym__expression] = "_expression",
  [sym_parenthesized] = "parenthesized",
  [sym__literal] = "_literal",
  [sym_boolean] = "boolean",
  [sym_list] = "list",
  [sym_map] = "map",
  [sym_entry] = "entry",
  [sym_block] = "block",
  [sym_call] = "call",
  [sym_function] = "function",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
  [aux_sym_list_repeat1] = "list_repeat1",
  [aux_sym_map_repeat1] = "map_repeat1",
  [aux_sym_block_repeat1] = "block_repeat1",
};

static TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_comment] = sym_comment,
  [anon_sym_COLON_EQ] = anon_sym_COLON_EQ,
  [anon_sym_SEMI] = anon_sym_SEMI,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [anon_sym_null] = anon_sym_null,
  [anon_sym_true] = anon_sym_true,
  [anon_sym_false] = anon_sym_false,
  [sym_integer] = sym_integer,
  [sym_string] = sym_string,
  [sym_symbol] = sym_symbol,
  [anon_sym_LBRACK] = anon_sym_LBRACK,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [anon_sym_RBRACK] = anon_sym_RBRACK,
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [anon_sym_COLON] = anon_sym_COLON,
  [sym_identifier] = sym_identifier,
  [anon_sym_EQ_GT] = anon_sym_EQ_GT,
  [sym_source_file] = sym_source_file,
  [sym_declaration] = sym_declaration,
  [sym_statement] = sym_statement,
  [sym__expression] = sym__expression,
  [sym_parenthesized] = sym_parenthesized,
  [sym__literal] = sym__literal,
  [sym_boolean] = sym_boolean,
  [sym_list] = sym_list,
  [sym_map] = sym_map,
  [sym_entry] = sym_entry,
  [sym_block] = sym_block,
  [sym_call] = sym_call,
  [sym_function] = sym_function,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
  [aux_sym_list_repeat1] = aux_sym_list_repeat1,
  [aux_sym_map_repeat1] = aux_sym_map_repeat1,
  [aux_sym_block_repeat1] = aux_sym_block_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [sym_comment] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_COLON_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SEMI] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_null] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_true] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_false] = {
    .visible = true,
    .named = false,
  },
  [sym_integer] = {
    .visible = true,
    .named = true,
  },
  [sym_string] = {
    .visible = true,
    .named = true,
  },
  [sym_symbol] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_LBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COMMA] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COLON] = {
    .visible = true,
    .named = false,
  },
  [sym_identifier] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_EQ_GT] = {
    .visible = true,
    .named = false,
  },
  [sym_source_file] = {
    .visible = true,
    .named = true,
  },
  [sym_declaration] = {
    .visible = true,
    .named = true,
  },
  [sym_statement] = {
    .visible = true,
    .named = true,
  },
  [sym__expression] = {
    .visible = false,
    .named = true,
  },
  [sym_parenthesized] = {
    .visible = true,
    .named = true,
  },
  [sym__literal] = {
    .visible = false,
    .named = true,
  },
  [sym_boolean] = {
    .visible = true,
    .named = true,
  },
  [sym_list] = {
    .visible = true,
    .named = true,
  },
  [sym_map] = {
    .visible = true,
    .named = true,
  },
  [sym_entry] = {
    .visible = true,
    .named = true,
  },
  [sym_block] = {
    .visible = true,
    .named = true,
  },
  [sym_call] = {
    .visible = true,
    .named = true,
  },
  [sym_function] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_source_file_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_list_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_map_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_block_repeat1] = {
    .visible = false,
    .named = false,
  },
};

enum {
  field_argument = 1,
  field_body = 2,
  field_declaration = 3,
  field_entry = 4,
  field_expression = 5,
  field_function = 6,
  field_item = 7,
  field_key = 8,
  field_name = 9,
  field_parameter = 10,
  field_statement = 11,
  field_value = 12,
};

static const char *ts_field_names[] = {
  [0] = NULL,
  [field_argument] = "argument",
  [field_body] = "body",
  [field_declaration] = "declaration",
  [field_entry] = "entry",
  [field_expression] = "expression",
  [field_function] = "function",
  [field_item] = "item",
  [field_key] = "key",
  [field_name] = "name",
  [field_parameter] = "parameter",
  [field_statement] = "statement",
  [field_value] = "value",
};

static const TSFieldMapSlice ts_field_map_slices[24] = {
  [1] = {.index = 0, .length = 1},
  [2] = {.index = 1, .length = 1},
  [3] = {.index = 2, .length = 2},
  [4] = {.index = 4, .length = 1},
  [5] = {.index = 5, .length = 2},
  [6] = {.index = 7, .length = 2},
  [7] = {.index = 9, .length = 1},
  [8] = {.index = 10, .length = 1},
  [9] = {.index = 11, .length = 1},
  [10] = {.index = 12, .length = 1},
  [11] = {.index = 13, .length = 2},
  [12] = {.index = 15, .length = 1},
  [13] = {.index = 16, .length = 1},
  [14] = {.index = 17, .length = 1},
  [15] = {.index = 18, .length = 1},
  [16] = {.index = 19, .length = 2},
  [17] = {.index = 21, .length = 1},
  [18] = {.index = 22, .length = 2},
  [19] = {.index = 24, .length = 2},
  [20] = {.index = 26, .length = 2},
  [21] = {.index = 28, .length = 2},
  [22] = {.index = 30, .length = 2},
  [23] = {.index = 32, .length = 2},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_declaration, 0},
  [1] =
    {field_declaration, 0, .inherited = true},
  [2] =
    {field_declaration, 0, .inherited = true},
    {field_declaration, 1, .inherited = true},
  [4] =
    {field_statement, 0},
  [5] =
    {field_name, 0},
    {field_value, 2},
  [7] =
    {field_argument, 1},
    {field_function, 0},
  [9] =
    {field_expression, 1},
  [10] =
    {field_item, 0},
  [11] =
    {field_item, 1},
  [12] =
    {field_item, 1, .inherited = true},
  [13] =
    {field_item, 0, .inherited = true},
    {field_item, 1, .inherited = true},
  [15] =
    {field_expression, 0},
  [16] =
    {field_entry, 0},
  [17] =
    {field_entry, 1},
  [18] =
    {field_entry, 1, .inherited = true},
  [19] =
    {field_entry, 0, .inherited = true},
    {field_entry, 1, .inherited = true},
  [21] =
    {field_statement, 1, .inherited = true},
  [22] =
    {field_statement, 0, .inherited = true},
    {field_statement, 1, .inherited = true},
  [24] =
    {field_body, 2},
    {field_parameter, 0},
  [26] =
    {field_item, 1, .inherited = true},
    {field_item, 2},
  [28] =
    {field_key, 0},
    {field_value, 2},
  [30] =
    {field_entry, 1, .inherited = true},
    {field_entry, 2},
  [32] =
    {field_expression, 2},
    {field_statement, 1, .inherited = true},
};

static TSSymbol ts_alias_sequences[24][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(8);
      if (lookahead == '"') ADVANCE(2);
      if (lookahead == '#') ADVANCE(9);
      if (lookahead == '(') ADVANCE(12);
      if (lookahead == ')') ADVANCE(13);
      if (lookahead == ',') ADVANCE(22);
      if (lookahead == '-') ADVANCE(5);
      if (lookahead == '.') ADVANCE(6);
      if (lookahead == ':') ADVANCE(27);
      if (lookahead == ';') ADVANCE(11);
      if (lookahead == '=') ADVANCE(4);
      if (lookahead == '[') ADVANCE(21);
      if (lookahead == ']') ADVANCE(23);
      if (lookahead == 'f') ADVANCE(28);
      if (lookahead == 'n') ADVANCE(37);
      if (lookahead == 't') ADVANCE(34);
      if (lookahead == '{') ADVANCE(24);
      if (lookahead == '}') ADVANCE(25);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(17);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(38);
      END_STATE();
    case 1:
      if (lookahead == '"') ADVANCE(2);
      if (lookahead == '#') ADVANCE(9);
      if (lookahead == '(') ADVANCE(12);
      if (lookahead == ')') ADVANCE(13);
      if (lookahead == ',') ADVANCE(22);
      if (lookahead == '-') ADVANCE(5);
      if (lookahead == '.') ADVANCE(6);
      if (lookahead == ':') ADVANCE(26);
      if (lookahead == ';') ADVANCE(11);
      if (lookahead == '=') ADVANCE(4);
      if (lookahead == '[') ADVANCE(21);
      if (lookahead == ']') ADVANCE(23);
      if (lookahead == 'f') ADVANCE(28);
      if (lookahead == 'n') ADVANCE(37);
      if (lookahead == 't') ADVANCE(34);
      if (lookahead == '{') ADVANCE(24);
      if (lookahead == '}') ADVANCE(25);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(1)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(17);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(38);
      END_STATE();
    case 2:
      if (lookahead == '"') ADVANCE(19);
      if (lookahead != 0) ADVANCE(2);
      END_STATE();
    case 3:
      if (lookahead == '=') ADVANCE(10);
      END_STATE();
    case 4:
      if (lookahead == '>') ADVANCE(39);
      END_STATE();
    case 5:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(18);
      END_STATE();
    case 6:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(20);
      END_STATE();
    case 7:
      if (eof) ADVANCE(8);
      if (lookahead == '#') ADVANCE(9);
      if (lookahead == ':') ADVANCE(3);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(7)
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(38);
      END_STATE();
    case 8:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 9:
      ACCEPT_TOKEN(sym_comment);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(9);
      END_STATE();
    case 10:
      ACCEPT_TOKEN(anon_sym_COLON_EQ);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(anon_sym_SEMI);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 13:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(anon_sym_null);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(38);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(anon_sym_true);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(38);
      END_STATE();
    case 16:
      ACCEPT_TOKEN(anon_sym_false);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(38);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(sym_integer);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(17);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(38);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(sym_integer);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(18);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(sym_string);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(sym_symbol);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(20);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 23:
      ACCEPT_TOKEN(anon_sym_RBRACK);
      END_STATE();
    case 24:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 25:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 26:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 27:
      ACCEPT_TOKEN(anon_sym_COLON);
      if (lookahead == '=') ADVANCE(10);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'a') ADVANCE(31);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(38);
      END_STATE();
    case 29:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(15);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(38);
      END_STATE();
    case 30:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(16);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(38);
      END_STATE();
    case 31:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(35);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(38);
      END_STATE();
    case 32:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(14);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(38);
      END_STATE();
    case 33:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(32);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(38);
      END_STATE();
    case 34:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'r') ADVANCE(36);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(38);
      END_STATE();
    case 35:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 's') ADVANCE(30);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(38);
      END_STATE();
    case 36:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'u') ADVANCE(29);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(38);
      END_STATE();
    case 37:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'u') ADVANCE(33);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(38);
      END_STATE();
    case 38:
      ACCEPT_TOKEN(sym_identifier);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(38);
      END_STATE();
    case 39:
      ACCEPT_TOKEN(anon_sym_EQ_GT);
      END_STATE();
    default:
      return false;
  }
}

static TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 7},
  [2] = {.lex_state = 1},
  [3] = {.lex_state = 1},
  [4] = {.lex_state = 0},
  [5] = {.lex_state = 0},
  [6] = {.lex_state = 0},
  [7] = {.lex_state = 0},
  [8] = {.lex_state = 0},
  [9] = {.lex_state = 0},
  [10] = {.lex_state = 0},
  [11] = {.lex_state = 0},
  [12] = {.lex_state = 0},
  [13] = {.lex_state = 0},
  [14] = {.lex_state = 0},
  [15] = {.lex_state = 1},
  [16] = {.lex_state = 0},
  [17] = {.lex_state = 1},
  [18] = {.lex_state = 0},
  [19] = {.lex_state = 0},
  [20] = {.lex_state = 0},
  [21] = {.lex_state = 0},
  [22] = {.lex_state = 0},
  [23] = {.lex_state = 0},
  [24] = {.lex_state = 0},
  [25] = {.lex_state = 0},
  [26] = {.lex_state = 1},
  [27] = {.lex_state = 1},
  [28] = {.lex_state = 1},
  [29] = {.lex_state = 1},
  [30] = {.lex_state = 1},
  [31] = {.lex_state = 1},
  [32] = {.lex_state = 1},
  [33] = {.lex_state = 1},
  [34] = {.lex_state = 1},
  [35] = {.lex_state = 1},
  [36] = {.lex_state = 1},
  [37] = {.lex_state = 1},
  [38] = {.lex_state = 1},
  [39] = {.lex_state = 0},
  [40] = {.lex_state = 0},
  [41] = {.lex_state = 0},
  [42] = {.lex_state = 0},
  [43] = {.lex_state = 7},
  [44] = {.lex_state = 7},
  [45] = {.lex_state = 7},
  [46] = {.lex_state = 7},
  [47] = {.lex_state = 0},
  [48] = {.lex_state = 0},
  [49] = {.lex_state = 0},
  [50] = {.lex_state = 0},
  [51] = {.lex_state = 7},
};

static uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_comment] = ACTIONS(3),
    [anon_sym_COLON_EQ] = ACTIONS(1),
    [anon_sym_SEMI] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [anon_sym_null] = ACTIONS(1),
    [anon_sym_true] = ACTIONS(1),
    [anon_sym_false] = ACTIONS(1),
    [sym_integer] = ACTIONS(1),
    [sym_string] = ACTIONS(1),
    [sym_symbol] = ACTIONS(1),
    [anon_sym_LBRACK] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [anon_sym_RBRACK] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [sym_identifier] = ACTIONS(1),
    [anon_sym_EQ_GT] = ACTIONS(1),
  },
  [1] = {
    [sym_source_file] = STATE(49),
    [sym_declaration] = STATE(45),
    [aux_sym_source_file_repeat1] = STATE(44),
    [ts_builtin_sym_end] = ACTIONS(5),
    [sym_comment] = ACTIONS(3),
    [sym_identifier] = ACTIONS(7),
  },
  [2] = {
    [sym__expression] = STATE(3),
    [sym_parenthesized] = STATE(3),
    [sym__literal] = STATE(3),
    [sym_boolean] = STATE(3),
    [sym_list] = STATE(3),
    [sym_map] = STATE(3),
    [sym_block] = STATE(3),
    [sym_call] = STATE(3),
    [sym_function] = STATE(3),
    [sym_comment] = ACTIONS(3),
    [anon_sym_SEMI] = ACTIONS(9),
    [anon_sym_LPAREN] = ACTIONS(11),
    [anon_sym_RPAREN] = ACTIONS(9),
    [anon_sym_null] = ACTIONS(13),
    [anon_sym_true] = ACTIONS(15),
    [anon_sym_false] = ACTIONS(15),
    [sym_integer] = ACTIONS(13),
    [sym_string] = ACTIONS(17),
    [sym_symbol] = ACTIONS(17),
    [anon_sym_LBRACK] = ACTIONS(19),
    [anon_sym_COMMA] = ACTIONS(9),
    [anon_sym_RBRACK] = ACTIONS(9),
    [anon_sym_LBRACE] = ACTIONS(21),
    [anon_sym_RBRACE] = ACTIONS(9),
    [anon_sym_COLON] = ACTIONS(9),
    [sym_identifier] = ACTIONS(23),
  },
  [3] = {
    [sym__expression] = STATE(3),
    [sym_parenthesized] = STATE(3),
    [sym__literal] = STATE(3),
    [sym_boolean] = STATE(3),
    [sym_list] = STATE(3),
    [sym_map] = STATE(3),
    [sym_block] = STATE(3),
    [sym_call] = STATE(3),
    [sym_function] = STATE(3),
    [sym_comment] = ACTIONS(3),
    [anon_sym_SEMI] = ACTIONS(25),
    [anon_sym_LPAREN] = ACTIONS(25),
    [anon_sym_RPAREN] = ACTIONS(25),
    [anon_sym_null] = ACTIONS(27),
    [anon_sym_true] = ACTIONS(27),
    [anon_sym_false] = ACTIONS(27),
    [sym_integer] = ACTIONS(27),
    [sym_string] = ACTIONS(25),
    [sym_symbol] = ACTIONS(25),
    [anon_sym_LBRACK] = ACTIONS(25),
    [anon_sym_COMMA] = ACTIONS(25),
    [anon_sym_RBRACK] = ACTIONS(25),
    [anon_sym_LBRACE] = ACTIONS(25),
    [anon_sym_RBRACE] = ACTIONS(25),
    [anon_sym_COLON] = ACTIONS(25),
    [sym_identifier] = ACTIONS(27),
  },
  [4] = {
    [sym_statement] = STATE(42),
    [sym__expression] = STATE(15),
    [sym_parenthesized] = STATE(15),
    [sym__literal] = STATE(15),
    [sym_boolean] = STATE(15),
    [sym_list] = STATE(15),
    [sym_map] = STATE(15),
    [sym_entry] = STATE(48),
    [sym_block] = STATE(15),
    [sym_call] = STATE(15),
    [sym_function] = STATE(15),
    [aux_sym_map_repeat1] = STATE(8),
    [aux_sym_block_repeat1] = STATE(7),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(11),
    [anon_sym_null] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(15),
    [anon_sym_false] = ACTIONS(15),
    [sym_integer] = ACTIONS(29),
    [sym_string] = ACTIONS(31),
    [sym_symbol] = ACTIONS(31),
    [anon_sym_LBRACK] = ACTIONS(19),
    [anon_sym_LBRACE] = ACTIONS(21),
    [anon_sym_RBRACE] = ACTIONS(33),
    [sym_identifier] = ACTIONS(23),
  },
  [5] = {
    [sym_statement] = STATE(42),
    [sym__expression] = STATE(20),
    [sym_parenthesized] = STATE(20),
    [sym__literal] = STATE(20),
    [sym_boolean] = STATE(20),
    [sym_list] = STATE(20),
    [sym_map] = STATE(20),
    [sym_block] = STATE(20),
    [sym_call] = STATE(20),
    [sym_function] = STATE(20),
    [aux_sym_block_repeat1] = STATE(5),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(35),
    [anon_sym_null] = ACTIONS(38),
    [anon_sym_true] = ACTIONS(41),
    [anon_sym_false] = ACTIONS(41),
    [sym_integer] = ACTIONS(38),
    [sym_string] = ACTIONS(44),
    [sym_symbol] = ACTIONS(44),
    [anon_sym_LBRACK] = ACTIONS(47),
    [anon_sym_LBRACE] = ACTIONS(50),
    [anon_sym_RBRACE] = ACTIONS(53),
    [sym_identifier] = ACTIONS(55),
  },
  [6] = {
    [sym__expression] = STATE(17),
    [sym_parenthesized] = STATE(17),
    [sym__literal] = STATE(17),
    [sym_boolean] = STATE(17),
    [sym_list] = STATE(17),
    [sym_map] = STATE(17),
    [sym_entry] = STATE(50),
    [sym_block] = STATE(17),
    [sym_call] = STATE(17),
    [sym_function] = STATE(17),
    [aux_sym_map_repeat1] = STATE(6),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(58),
    [anon_sym_null] = ACTIONS(61),
    [anon_sym_true] = ACTIONS(64),
    [anon_sym_false] = ACTIONS(64),
    [sym_integer] = ACTIONS(61),
    [sym_string] = ACTIONS(67),
    [sym_symbol] = ACTIONS(67),
    [anon_sym_LBRACK] = ACTIONS(70),
    [anon_sym_LBRACE] = ACTIONS(73),
    [anon_sym_RBRACE] = ACTIONS(76),
    [sym_identifier] = ACTIONS(78),
  },
  [7] = {
    [sym_statement] = STATE(42),
    [sym__expression] = STATE(11),
    [sym_parenthesized] = STATE(11),
    [sym__literal] = STATE(11),
    [sym_boolean] = STATE(11),
    [sym_list] = STATE(11),
    [sym_map] = STATE(11),
    [sym_block] = STATE(11),
    [sym_call] = STATE(11),
    [sym_function] = STATE(11),
    [aux_sym_block_repeat1] = STATE(5),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(11),
    [anon_sym_null] = ACTIONS(81),
    [anon_sym_true] = ACTIONS(15),
    [anon_sym_false] = ACTIONS(15),
    [sym_integer] = ACTIONS(81),
    [sym_string] = ACTIONS(83),
    [sym_symbol] = ACTIONS(83),
    [anon_sym_LBRACK] = ACTIONS(19),
    [anon_sym_LBRACE] = ACTIONS(21),
    [anon_sym_RBRACE] = ACTIONS(85),
    [sym_identifier] = ACTIONS(23),
  },
  [8] = {
    [sym__expression] = STATE(17),
    [sym_parenthesized] = STATE(17),
    [sym__literal] = STATE(17),
    [sym_boolean] = STATE(17),
    [sym_list] = STATE(17),
    [sym_map] = STATE(17),
    [sym_entry] = STATE(47),
    [sym_block] = STATE(17),
    [sym_call] = STATE(17),
    [sym_function] = STATE(17),
    [aux_sym_map_repeat1] = STATE(6),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(11),
    [anon_sym_null] = ACTIONS(87),
    [anon_sym_true] = ACTIONS(15),
    [anon_sym_false] = ACTIONS(15),
    [sym_integer] = ACTIONS(87),
    [sym_string] = ACTIONS(89),
    [sym_symbol] = ACTIONS(89),
    [anon_sym_LBRACK] = ACTIONS(19),
    [anon_sym_LBRACE] = ACTIONS(21),
    [anon_sym_RBRACE] = ACTIONS(91),
    [sym_identifier] = ACTIONS(23),
  },
  [9] = {
    [sym__expression] = STATE(13),
    [sym_parenthesized] = STATE(13),
    [sym__literal] = STATE(13),
    [sym_boolean] = STATE(13),
    [sym_list] = STATE(13),
    [sym_map] = STATE(13),
    [sym_block] = STATE(13),
    [sym_call] = STATE(13),
    [sym_function] = STATE(13),
    [aux_sym_list_repeat1] = STATE(12),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(11),
    [anon_sym_null] = ACTIONS(93),
    [anon_sym_true] = ACTIONS(15),
    [anon_sym_false] = ACTIONS(15),
    [sym_integer] = ACTIONS(93),
    [sym_string] = ACTIONS(95),
    [sym_symbol] = ACTIONS(95),
    [anon_sym_LBRACK] = ACTIONS(19),
    [anon_sym_RBRACK] = ACTIONS(97),
    [anon_sym_LBRACE] = ACTIONS(21),
    [sym_identifier] = ACTIONS(23),
  },
  [10] = {
    [sym__expression] = STATE(3),
    [sym_parenthesized] = STATE(3),
    [sym__literal] = STATE(3),
    [sym_boolean] = STATE(3),
    [sym_list] = STATE(3),
    [sym_map] = STATE(3),
    [sym_block] = STATE(3),
    [sym_call] = STATE(3),
    [sym_function] = STATE(3),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(11),
    [anon_sym_null] = ACTIONS(13),
    [anon_sym_true] = ACTIONS(15),
    [anon_sym_false] = ACTIONS(15),
    [sym_integer] = ACTIONS(13),
    [sym_string] = ACTIONS(17),
    [sym_symbol] = ACTIONS(17),
    [anon_sym_LBRACK] = ACTIONS(19),
    [anon_sym_COMMA] = ACTIONS(99),
    [anon_sym_LBRACE] = ACTIONS(21),
    [anon_sym_RBRACE] = ACTIONS(99),
    [sym_identifier] = ACTIONS(23),
  },
  [11] = {
    [sym__expression] = STATE(3),
    [sym_parenthesized] = STATE(3),
    [sym__literal] = STATE(3),
    [sym_boolean] = STATE(3),
    [sym_list] = STATE(3),
    [sym_map] = STATE(3),
    [sym_block] = STATE(3),
    [sym_call] = STATE(3),
    [sym_function] = STATE(3),
    [sym_comment] = ACTIONS(3),
    [anon_sym_SEMI] = ACTIONS(101),
    [anon_sym_LPAREN] = ACTIONS(11),
    [anon_sym_null] = ACTIONS(13),
    [anon_sym_true] = ACTIONS(15),
    [anon_sym_false] = ACTIONS(15),
    [sym_integer] = ACTIONS(13),
    [sym_string] = ACTIONS(17),
    [sym_symbol] = ACTIONS(17),
    [anon_sym_LBRACK] = ACTIONS(19),
    [anon_sym_LBRACE] = ACTIONS(21),
    [anon_sym_RBRACE] = ACTIONS(103),
    [sym_identifier] = ACTIONS(23),
  },
  [12] = {
    [sym__expression] = STATE(21),
    [sym_parenthesized] = STATE(21),
    [sym__literal] = STATE(21),
    [sym_boolean] = STATE(21),
    [sym_list] = STATE(21),
    [sym_map] = STATE(21),
    [sym_block] = STATE(21),
    [sym_call] = STATE(21),
    [sym_function] = STATE(21),
    [aux_sym_list_repeat1] = STATE(12),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(105),
    [anon_sym_null] = ACTIONS(108),
    [anon_sym_true] = ACTIONS(111),
    [anon_sym_false] = ACTIONS(111),
    [sym_integer] = ACTIONS(108),
    [sym_string] = ACTIONS(114),
    [sym_symbol] = ACTIONS(114),
    [anon_sym_LBRACK] = ACTIONS(117),
    [anon_sym_RBRACK] = ACTIONS(120),
    [anon_sym_LBRACE] = ACTIONS(122),
    [sym_identifier] = ACTIONS(125),
  },
  [13] = {
    [sym__expression] = STATE(3),
    [sym_parenthesized] = STATE(3),
    [sym__literal] = STATE(3),
    [sym_boolean] = STATE(3),
    [sym_list] = STATE(3),
    [sym_map] = STATE(3),
    [sym_block] = STATE(3),
    [sym_call] = STATE(3),
    [sym_function] = STATE(3),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(11),
    [anon_sym_null] = ACTIONS(13),
    [anon_sym_true] = ACTIONS(15),
    [anon_sym_false] = ACTIONS(15),
    [sym_integer] = ACTIONS(13),
    [sym_string] = ACTIONS(17),
    [sym_symbol] = ACTIONS(17),
    [anon_sym_LBRACK] = ACTIONS(19),
    [anon_sym_COMMA] = ACTIONS(128),
    [anon_sym_RBRACK] = ACTIONS(130),
    [anon_sym_LBRACE] = ACTIONS(21),
    [sym_identifier] = ACTIONS(23),
  },
  [14] = {
    [sym__expression] = STATE(16),
    [sym_parenthesized] = STATE(16),
    [sym__literal] = STATE(16),
    [sym_boolean] = STATE(16),
    [sym_list] = STATE(16),
    [sym_map] = STATE(16),
    [sym_block] = STATE(16),
    [sym_call] = STATE(16),
    [sym_function] = STATE(16),
    [aux_sym_list_repeat1] = STATE(9),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(11),
    [anon_sym_null] = ACTIONS(132),
    [anon_sym_true] = ACTIONS(15),
    [anon_sym_false] = ACTIONS(15),
    [sym_integer] = ACTIONS(132),
    [sym_string] = ACTIONS(134),
    [sym_symbol] = ACTIONS(134),
    [anon_sym_LBRACK] = ACTIONS(19),
    [anon_sym_RBRACK] = ACTIONS(136),
    [anon_sym_LBRACE] = ACTIONS(21),
    [sym_identifier] = ACTIONS(23),
  },
  [15] = {
    [sym__expression] = STATE(3),
    [sym_parenthesized] = STATE(3),
    [sym__literal] = STATE(3),
    [sym_boolean] = STATE(3),
    [sym_list] = STATE(3),
    [sym_map] = STATE(3),
    [sym_block] = STATE(3),
    [sym_call] = STATE(3),
    [sym_function] = STATE(3),
    [sym_comment] = ACTIONS(3),
    [anon_sym_SEMI] = ACTIONS(101),
    [anon_sym_LPAREN] = ACTIONS(11),
    [anon_sym_null] = ACTIONS(13),
    [anon_sym_true] = ACTIONS(15),
    [anon_sym_false] = ACTIONS(15),
    [sym_integer] = ACTIONS(13),
    [sym_string] = ACTIONS(17),
    [sym_symbol] = ACTIONS(17),
    [anon_sym_LBRACK] = ACTIONS(19),
    [anon_sym_LBRACE] = ACTIONS(21),
    [anon_sym_COLON] = ACTIONS(138),
    [sym_identifier] = ACTIONS(23),
  },
  [16] = {
    [sym__expression] = STATE(3),
    [sym_parenthesized] = STATE(3),
    [sym__literal] = STATE(3),
    [sym_boolean] = STATE(3),
    [sym_list] = STATE(3),
    [sym_map] = STATE(3),
    [sym_block] = STATE(3),
    [sym_call] = STATE(3),
    [sym_function] = STATE(3),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(11),
    [anon_sym_null] = ACTIONS(13),
    [anon_sym_true] = ACTIONS(15),
    [anon_sym_false] = ACTIONS(15),
    [sym_integer] = ACTIONS(13),
    [sym_string] = ACTIONS(17),
    [sym_symbol] = ACTIONS(17),
    [anon_sym_LBRACK] = ACTIONS(19),
    [anon_sym_COMMA] = ACTIONS(128),
    [anon_sym_RBRACK] = ACTIONS(140),
    [anon_sym_LBRACE] = ACTIONS(21),
    [sym_identifier] = ACTIONS(23),
  },
  [17] = {
    [sym__expression] = STATE(3),
    [sym_parenthesized] = STATE(3),
    [sym__literal] = STATE(3),
    [sym_boolean] = STATE(3),
    [sym_list] = STATE(3),
    [sym_map] = STATE(3),
    [sym_block] = STATE(3),
    [sym_call] = STATE(3),
    [sym_function] = STATE(3),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(11),
    [anon_sym_null] = ACTIONS(13),
    [anon_sym_true] = ACTIONS(15),
    [anon_sym_false] = ACTIONS(15),
    [sym_integer] = ACTIONS(13),
    [sym_string] = ACTIONS(17),
    [sym_symbol] = ACTIONS(17),
    [anon_sym_LBRACK] = ACTIONS(19),
    [anon_sym_LBRACE] = ACTIONS(21),
    [anon_sym_COLON] = ACTIONS(138),
    [sym_identifier] = ACTIONS(23),
  },
  [18] = {
    [sym__expression] = STATE(3),
    [sym_parenthesized] = STATE(3),
    [sym__literal] = STATE(3),
    [sym_boolean] = STATE(3),
    [sym_list] = STATE(3),
    [sym_map] = STATE(3),
    [sym_block] = STATE(3),
    [sym_call] = STATE(3),
    [sym_function] = STATE(3),
    [sym_comment] = ACTIONS(3),
    [anon_sym_SEMI] = ACTIONS(142),
    [anon_sym_LPAREN] = ACTIONS(11),
    [anon_sym_null] = ACTIONS(13),
    [anon_sym_true] = ACTIONS(15),
    [anon_sym_false] = ACTIONS(15),
    [sym_integer] = ACTIONS(13),
    [sym_string] = ACTIONS(17),
    [sym_symbol] = ACTIONS(17),
    [anon_sym_LBRACK] = ACTIONS(19),
    [anon_sym_LBRACE] = ACTIONS(21),
    [sym_identifier] = ACTIONS(23),
  },
  [19] = {
    [sym__expression] = STATE(3),
    [sym_parenthesized] = STATE(3),
    [sym__literal] = STATE(3),
    [sym_boolean] = STATE(3),
    [sym_list] = STATE(3),
    [sym_map] = STATE(3),
    [sym_block] = STATE(3),
    [sym_call] = STATE(3),
    [sym_function] = STATE(3),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(11),
    [anon_sym_RPAREN] = ACTIONS(144),
    [anon_sym_null] = ACTIONS(13),
    [anon_sym_true] = ACTIONS(15),
    [anon_sym_false] = ACTIONS(15),
    [sym_integer] = ACTIONS(13),
    [sym_string] = ACTIONS(17),
    [sym_symbol] = ACTIONS(17),
    [anon_sym_LBRACK] = ACTIONS(19),
    [anon_sym_LBRACE] = ACTIONS(21),
    [sym_identifier] = ACTIONS(23),
  },
  [20] = {
    [sym__expression] = STATE(3),
    [sym_parenthesized] = STATE(3),
    [sym__literal] = STATE(3),
    [sym_boolean] = STATE(3),
    [sym_list] = STATE(3),
    [sym_map] = STATE(3),
    [sym_block] = STATE(3),
    [sym_call] = STATE(3),
    [sym_function] = STATE(3),
    [sym_comment] = ACTIONS(3),
    [anon_sym_SEMI] = ACTIONS(101),
    [anon_sym_LPAREN] = ACTIONS(11),
    [anon_sym_null] = ACTIONS(13),
    [anon_sym_true] = ACTIONS(15),
    [anon_sym_false] = ACTIONS(15),
    [sym_integer] = ACTIONS(13),
    [sym_string] = ACTIONS(17),
    [sym_symbol] = ACTIONS(17),
    [anon_sym_LBRACK] = ACTIONS(19),
    [anon_sym_LBRACE] = ACTIONS(21),
    [sym_identifier] = ACTIONS(23),
  },
  [21] = {
    [sym__expression] = STATE(3),
    [sym_parenthesized] = STATE(3),
    [sym__literal] = STATE(3),
    [sym_boolean] = STATE(3),
    [sym_list] = STATE(3),
    [sym_map] = STATE(3),
    [sym_block] = STATE(3),
    [sym_call] = STATE(3),
    [sym_function] = STATE(3),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(11),
    [anon_sym_null] = ACTIONS(13),
    [anon_sym_true] = ACTIONS(15),
    [anon_sym_false] = ACTIONS(15),
    [sym_integer] = ACTIONS(13),
    [sym_string] = ACTIONS(17),
    [sym_symbol] = ACTIONS(17),
    [anon_sym_LBRACK] = ACTIONS(19),
    [anon_sym_COMMA] = ACTIONS(128),
    [anon_sym_LBRACE] = ACTIONS(21),
    [sym_identifier] = ACTIONS(23),
  },
  [22] = {
    [sym__expression] = STATE(18),
    [sym_parenthesized] = STATE(18),
    [sym__literal] = STATE(18),
    [sym_boolean] = STATE(18),
    [sym_list] = STATE(18),
    [sym_map] = STATE(18),
    [sym_block] = STATE(18),
    [sym_call] = STATE(18),
    [sym_function] = STATE(18),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(11),
    [anon_sym_null] = ACTIONS(146),
    [anon_sym_true] = ACTIONS(15),
    [anon_sym_false] = ACTIONS(15),
    [sym_integer] = ACTIONS(146),
    [sym_string] = ACTIONS(148),
    [sym_symbol] = ACTIONS(148),
    [anon_sym_LBRACK] = ACTIONS(19),
    [anon_sym_LBRACE] = ACTIONS(21),
    [sym_identifier] = ACTIONS(23),
  },
  [23] = {
    [sym__expression] = STATE(2),
    [sym_parenthesized] = STATE(2),
    [sym__literal] = STATE(2),
    [sym_boolean] = STATE(2),
    [sym_list] = STATE(2),
    [sym_map] = STATE(2),
    [sym_block] = STATE(2),
    [sym_call] = STATE(2),
    [sym_function] = STATE(2),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(11),
    [anon_sym_null] = ACTIONS(150),
    [anon_sym_true] = ACTIONS(15),
    [anon_sym_false] = ACTIONS(15),
    [sym_integer] = ACTIONS(150),
    [sym_string] = ACTIONS(152),
    [sym_symbol] = ACTIONS(152),
    [anon_sym_LBRACK] = ACTIONS(19),
    [anon_sym_LBRACE] = ACTIONS(21),
    [sym_identifier] = ACTIONS(23),
  },
  [24] = {
    [sym__expression] = STATE(19),
    [sym_parenthesized] = STATE(19),
    [sym__literal] = STATE(19),
    [sym_boolean] = STATE(19),
    [sym_list] = STATE(19),
    [sym_map] = STATE(19),
    [sym_block] = STATE(19),
    [sym_call] = STATE(19),
    [sym_function] = STATE(19),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(11),
    [anon_sym_null] = ACTIONS(154),
    [anon_sym_true] = ACTIONS(15),
    [anon_sym_false] = ACTIONS(15),
    [sym_integer] = ACTIONS(154),
    [sym_string] = ACTIONS(156),
    [sym_symbol] = ACTIONS(156),
    [anon_sym_LBRACK] = ACTIONS(19),
    [anon_sym_LBRACE] = ACTIONS(21),
    [sym_identifier] = ACTIONS(23),
  },
  [25] = {
    [sym__expression] = STATE(10),
    [sym_parenthesized] = STATE(10),
    [sym__literal] = STATE(10),
    [sym_boolean] = STATE(10),
    [sym_list] = STATE(10),
    [sym_map] = STATE(10),
    [sym_block] = STATE(10),
    [sym_call] = STATE(10),
    [sym_function] = STATE(10),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(11),
    [anon_sym_null] = ACTIONS(158),
    [anon_sym_true] = ACTIONS(15),
    [anon_sym_false] = ACTIONS(15),
    [sym_integer] = ACTIONS(158),
    [sym_string] = ACTIONS(160),
    [sym_symbol] = ACTIONS(160),
    [anon_sym_LBRACK] = ACTIONS(19),
    [anon_sym_LBRACE] = ACTIONS(21),
    [sym_identifier] = ACTIONS(23),
  },
};

static uint16_t ts_small_parse_table[] = {
  [0] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(166), 1,
      anon_sym_EQ_GT,
    ACTIONS(164), 5,
      anon_sym_null,
      anon_sym_true,
      anon_sym_false,
      sym_integer,
      sym_identifier,
    ACTIONS(162), 11,
      anon_sym_SEMI,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_string,
      sym_symbol,
      anon_sym_LBRACK,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_COLON,
  [27] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(170), 5,
      anon_sym_null,
      anon_sym_true,
      anon_sym_false,
      sym_integer,
      sym_identifier,
    ACTIONS(168), 11,
      anon_sym_SEMI,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_string,
      sym_symbol,
      anon_sym_LBRACK,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_COLON,
  [51] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(174), 5,
      anon_sym_null,
      anon_sym_true,
      anon_sym_false,
      sym_integer,
      sym_identifier,
    ACTIONS(172), 11,
      anon_sym_SEMI,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_string,
      sym_symbol,
      anon_sym_LBRACK,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_COLON,
  [75] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(178), 5,
      anon_sym_null,
      anon_sym_true,
      anon_sym_false,
      sym_integer,
      sym_identifier,
    ACTIONS(176), 11,
      anon_sym_SEMI,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_string,
      sym_symbol,
      anon_sym_LBRACK,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_COLON,
  [99] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(182), 5,
      anon_sym_null,
      anon_sym_true,
      anon_sym_false,
      sym_integer,
      sym_identifier,
    ACTIONS(180), 11,
      anon_sym_SEMI,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_string,
      sym_symbol,
      anon_sym_LBRACK,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_COLON,
  [123] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(186), 5,
      anon_sym_null,
      anon_sym_true,
      anon_sym_false,
      sym_integer,
      sym_identifier,
    ACTIONS(184), 11,
      anon_sym_SEMI,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_string,
      sym_symbol,
      anon_sym_LBRACK,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_COLON,
  [147] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(190), 5,
      anon_sym_null,
      anon_sym_true,
      anon_sym_false,
      sym_integer,
      sym_identifier,
    ACTIONS(188), 11,
      anon_sym_SEMI,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_string,
      sym_symbol,
      anon_sym_LBRACK,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_COLON,
  [171] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(194), 5,
      anon_sym_null,
      anon_sym_true,
      anon_sym_false,
      sym_integer,
      sym_identifier,
    ACTIONS(192), 11,
      anon_sym_SEMI,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_string,
      sym_symbol,
      anon_sym_LBRACK,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_COLON,
  [195] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(198), 5,
      anon_sym_null,
      anon_sym_true,
      anon_sym_false,
      sym_integer,
      sym_identifier,
    ACTIONS(196), 11,
      anon_sym_SEMI,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_string,
      sym_symbol,
      anon_sym_LBRACK,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_COLON,
  [219] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(202), 5,
      anon_sym_null,
      anon_sym_true,
      anon_sym_false,
      sym_integer,
      sym_identifier,
    ACTIONS(200), 11,
      anon_sym_SEMI,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_string,
      sym_symbol,
      anon_sym_LBRACK,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_COLON,
  [243] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(206), 5,
      anon_sym_null,
      anon_sym_true,
      anon_sym_false,
      sym_integer,
      sym_identifier,
    ACTIONS(204), 11,
      anon_sym_SEMI,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_string,
      sym_symbol,
      anon_sym_LBRACK,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_COLON,
  [267] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(210), 5,
      anon_sym_null,
      anon_sym_true,
      anon_sym_false,
      sym_integer,
      sym_identifier,
    ACTIONS(208), 11,
      anon_sym_SEMI,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_string,
      sym_symbol,
      anon_sym_LBRACK,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_COLON,
  [291] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(214), 5,
      anon_sym_null,
      anon_sym_true,
      anon_sym_false,
      sym_integer,
      sym_identifier,
    ACTIONS(212), 11,
      anon_sym_SEMI,
      anon_sym_LPAREN,
      anon_sym_RPAREN,
      sym_string,
      sym_symbol,
      anon_sym_LBRACK,
      anon_sym_COMMA,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_COLON,
  [315] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(218), 5,
      anon_sym_null,
      anon_sym_true,
      anon_sym_false,
      sym_integer,
      sym_identifier,
    ACTIONS(216), 6,
      anon_sym_LPAREN,
      sym_string,
      sym_symbol,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_LBRACE,
  [334] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(222), 5,
      anon_sym_null,
      anon_sym_true,
      anon_sym_false,
      sym_integer,
      sym_identifier,
    ACTIONS(220), 6,
      anon_sym_LPAREN,
      sym_string,
      sym_symbol,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
  [353] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(226), 5,
      anon_sym_null,
      anon_sym_true,
      anon_sym_false,
      sym_integer,
      sym_identifier,
    ACTIONS(224), 6,
      anon_sym_LPAREN,
      sym_string,
      sym_symbol,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
  [372] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(230), 5,
      anon_sym_null,
      anon_sym_true,
      anon_sym_false,
      sym_integer,
      sym_identifier,
    ACTIONS(228), 6,
      anon_sym_LPAREN,
      sym_string,
      sym_symbol,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
  [391] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(232), 1,
      ts_builtin_sym_end,
    ACTIONS(234), 1,
      sym_identifier,
    STATE(43), 1,
      aux_sym_source_file_repeat1,
    STATE(45), 1,
      sym_declaration,
  [407] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(7), 1,
      sym_identifier,
    ACTIONS(237), 1,
      ts_builtin_sym_end,
    STATE(43), 1,
      aux_sym_source_file_repeat1,
    STATE(45), 1,
      sym_declaration,
  [423] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(239), 2,
      ts_builtin_sym_end,
      sym_identifier,
  [431] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(241), 2,
      ts_builtin_sym_end,
      sym_identifier,
  [439] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(243), 1,
      anon_sym_COMMA,
    ACTIONS(245), 1,
      anon_sym_RBRACE,
  [449] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(243), 1,
      anon_sym_COMMA,
    ACTIONS(247), 1,
      anon_sym_RBRACE,
  [459] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(249), 1,
      ts_builtin_sym_end,
  [466] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(243), 1,
      anon_sym_COMMA,
  [473] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(251), 1,
      anon_sym_COLON_EQ,
};

static uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(26)] = 0,
  [SMALL_STATE(27)] = 27,
  [SMALL_STATE(28)] = 51,
  [SMALL_STATE(29)] = 75,
  [SMALL_STATE(30)] = 99,
  [SMALL_STATE(31)] = 123,
  [SMALL_STATE(32)] = 147,
  [SMALL_STATE(33)] = 171,
  [SMALL_STATE(34)] = 195,
  [SMALL_STATE(35)] = 219,
  [SMALL_STATE(36)] = 243,
  [SMALL_STATE(37)] = 267,
  [SMALL_STATE(38)] = 291,
  [SMALL_STATE(39)] = 315,
  [SMALL_STATE(40)] = 334,
  [SMALL_STATE(41)] = 353,
  [SMALL_STATE(42)] = 372,
  [SMALL_STATE(43)] = 391,
  [SMALL_STATE(44)] = 407,
  [SMALL_STATE(45)] = 423,
  [SMALL_STATE(46)] = 431,
  [SMALL_STATE(47)] = 439,
  [SMALL_STATE(48)] = 449,
  [SMALL_STATE(49)] = 459,
  [SMALL_STATE(50)] = 466,
  [SMALL_STATE(51)] = 473,
};

static TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(51),
  [9] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function, 3, .production_id = 19),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [13] = {.entry = {.count = 1, .reusable = false}}, SHIFT(3),
  [15] = {.entry = {.count = 1, .reusable = false}}, SHIFT(31),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [21] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [23] = {.entry = {.count = 1, .reusable = false}}, SHIFT(26),
  [25] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_call, 2, .production_id = 6),
  [27] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_call, 2, .production_id = 6),
  [29] = {.entry = {.count = 1, .reusable = false}}, SHIFT(15),
  [31] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [33] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [35] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 18), SHIFT_REPEAT(24),
  [38] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 18), SHIFT_REPEAT(20),
  [41] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 18), SHIFT_REPEAT(31),
  [44] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 18), SHIFT_REPEAT(20),
  [47] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 18), SHIFT_REPEAT(14),
  [50] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 18), SHIFT_REPEAT(4),
  [53] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 18),
  [55] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 2, .production_id = 18), SHIFT_REPEAT(26),
  [58] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_map_repeat1, 2, .production_id = 16), SHIFT_REPEAT(24),
  [61] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_map_repeat1, 2, .production_id = 16), SHIFT_REPEAT(17),
  [64] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_map_repeat1, 2, .production_id = 16), SHIFT_REPEAT(31),
  [67] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_map_repeat1, 2, .production_id = 16), SHIFT_REPEAT(17),
  [70] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_map_repeat1, 2, .production_id = 16), SHIFT_REPEAT(14),
  [73] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_map_repeat1, 2, .production_id = 16), SHIFT_REPEAT(4),
  [76] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_map_repeat1, 2, .production_id = 16),
  [78] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_map_repeat1, 2, .production_id = 16), SHIFT_REPEAT(26),
  [81] = {.entry = {.count = 1, .reusable = false}}, SHIFT(11),
  [83] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [85] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [87] = {.entry = {.count = 1, .reusable = false}}, SHIFT(17),
  [89] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [91] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [93] = {.entry = {.count = 1, .reusable = false}}, SHIFT(13),
  [95] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [97] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [99] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_entry, 3, .production_id = 21),
  [101] = {.entry = {.count = 1, .reusable = true}}, SHIFT(40),
  [103] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [105] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_list_repeat1, 2, .production_id = 11), SHIFT_REPEAT(24),
  [108] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_list_repeat1, 2, .production_id = 11), SHIFT_REPEAT(21),
  [111] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_list_repeat1, 2, .production_id = 11), SHIFT_REPEAT(31),
  [114] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_list_repeat1, 2, .production_id = 11), SHIFT_REPEAT(21),
  [117] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_list_repeat1, 2, .production_id = 11), SHIFT_REPEAT(14),
  [120] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_list_repeat1, 2, .production_id = 11),
  [122] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_list_repeat1, 2, .production_id = 11), SHIFT_REPEAT(4),
  [125] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_list_repeat1, 2, .production_id = 11), SHIFT_REPEAT(26),
  [128] = {.entry = {.count = 1, .reusable = true}}, SHIFT(39),
  [130] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [132] = {.entry = {.count = 1, .reusable = false}}, SHIFT(16),
  [134] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [136] = {.entry = {.count = 1, .reusable = true}}, SHIFT(38),
  [138] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [140] = {.entry = {.count = 1, .reusable = true}}, SHIFT(29),
  [142] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [144] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [146] = {.entry = {.count = 1, .reusable = false}}, SHIFT(18),
  [148] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [150] = {.entry = {.count = 1, .reusable = false}}, SHIFT(2),
  [152] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [154] = {.entry = {.count = 1, .reusable = false}}, SHIFT(19),
  [156] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [158] = {.entry = {.count = 1, .reusable = false}}, SHIFT(10),
  [160] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [162] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__expression, 1),
  [164] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__expression, 1),
  [166] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [168] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_map, 3, .production_id = 15),
  [170] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_map, 3, .production_id = 15),
  [172] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_list, 4, .production_id = 20),
  [174] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_list, 4, .production_id = 20),
  [176] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_list, 3, .production_id = 9),
  [178] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_list, 3, .production_id = 9),
  [180] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_list, 3, .production_id = 10),
  [182] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_list, 3, .production_id = 10),
  [184] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_boolean, 1),
  [186] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_boolean, 1),
  [188] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_map, 2),
  [190] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_map, 2),
  [192] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_map, 3, .production_id = 14),
  [194] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_map, 3, .production_id = 14),
  [196] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 4, .production_id = 23),
  [198] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_block, 4, .production_id = 23),
  [200] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_map, 4, .production_id = 22),
  [202] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_map, 4, .production_id = 22),
  [204] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parenthesized, 3, .production_id = 7),
  [206] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_parenthesized, 3, .production_id = 7),
  [208] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 3, .production_id = 17),
  [210] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_block, 3, .production_id = 17),
  [212] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_list, 2),
  [214] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_list, 2),
  [216] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_list_repeat1, 2, .production_id = 8),
  [218] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_list_repeat1, 2, .production_id = 8),
  [220] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_statement, 2, .production_id = 12),
  [222] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_statement, 2, .production_id = 12),
  [224] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_map_repeat1, 2, .production_id = 13),
  [226] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_map_repeat1, 2, .production_id = 13),
  [228] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_block_repeat1, 1, .production_id = 4),
  [230] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_block_repeat1, 1, .production_id = 4),
  [232] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, .production_id = 3),
  [234] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, .production_id = 3), SHIFT_REPEAT(51),
  [237] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1, .production_id = 2),
  [239] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 1, .production_id = 1),
  [241] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_declaration, 4, .production_id = 5),
  [243] = {.entry = {.count = 1, .reusable = true}}, SHIFT(41),
  [245] = {.entry = {.count = 1, .reusable = true}}, SHIFT(35),
  [247] = {.entry = {.count = 1, .reusable = true}}, SHIFT(33),
  [249] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [251] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_quench(void) {
  static TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .symbol_names = ts_symbol_names,
    .symbol_metadata = ts_symbol_metadata,
    .parse_table = (const uint16_t *)ts_parse_table,
    .parse_actions = ts_parse_actions,
    .lex_modes = ts_lex_modes,
    .alias_sequences = (const TSSymbol *)ts_alias_sequences,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .lex_fn = ts_lex,
    .field_count = FIELD_COUNT,
    .field_map_slices = (const TSFieldMapSlice *)ts_field_map_slices,
    .field_map_entries = (const TSFieldMapEntry *)ts_field_map_entries,
    .field_names = ts_field_names,
    .large_state_count = LARGE_STATE_COUNT,
    .small_parse_table = (const uint16_t *)ts_small_parse_table,
    .small_parse_table_map = (const uint32_t *)ts_small_parse_table_map,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .state_count = STATE_COUNT,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
