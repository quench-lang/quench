#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 12
#define STATE_COUNT 19
#define LARGE_STATE_COUNT 4
#define SYMBOL_COUNT 16
#define ALIAS_COUNT 0
#define TOKEN_COUNT 8
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 5
#define MAX_ALIAS_SEQUENCE_LENGTH 4

enum {
  sym_comment = 1,
  anon_sym_SEMI = 2,
  sym_string = 3,
  sym_identifier = 4,
  anon_sym_LPAREN = 5,
  anon_sym_RPAREN = 6,
  anon_sym_COMMA = 7,
  sym_source_file = 8,
  sym_statement = 9,
  sym__expression = 10,
  sym__literal = 11,
  sym_call = 12,
  sym_arguments = 13,
  aux_sym_source_file_repeat1 = 14,
  aux_sym_arguments_repeat1 = 15,
};

static const char *ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_comment] = "comment",
  [anon_sym_SEMI] = ";",
  [sym_string] = "string",
  [sym_identifier] = "identifier",
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [anon_sym_COMMA] = ",",
  [sym_source_file] = "source_file",
  [sym_statement] = "statement",
  [sym__expression] = "_expression",
  [sym__literal] = "_literal",
  [sym_call] = "call",
  [sym_arguments] = "arguments",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
  [aux_sym_arguments_repeat1] = "arguments_repeat1",
};

static TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_comment] = sym_comment,
  [anon_sym_SEMI] = anon_sym_SEMI,
  [sym_string] = sym_string,
  [sym_identifier] = sym_identifier,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [sym_source_file] = sym_source_file,
  [sym_statement] = sym_statement,
  [sym__expression] = sym__expression,
  [sym__literal] = sym__literal,
  [sym_call] = sym_call,
  [sym_arguments] = sym_arguments,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
  [aux_sym_arguments_repeat1] = aux_sym_arguments_repeat1,
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
  [anon_sym_SEMI] = {
    .visible = true,
    .named = false,
  },
  [sym_string] = {
    .visible = true,
    .named = true,
  },
  [sym_identifier] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COMMA] = {
    .visible = true,
    .named = false,
  },
  [sym_source_file] = {
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
  [sym__literal] = {
    .visible = false,
    .named = true,
  },
  [sym_call] = {
    .visible = true,
    .named = true,
  },
  [sym_arguments] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_source_file_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_arguments_repeat1] = {
    .visible = false,
    .named = false,
  },
};

enum {
  field_argument = 1,
  field_arguments = 2,
  field_body = 3,
  field_expression = 4,
  field_function = 5,
};

static const char *ts_field_names[] = {
  [0] = NULL,
  [field_argument] = "argument",
  [field_arguments] = "arguments",
  [field_body] = "body",
  [field_expression] = "expression",
  [field_function] = "function",
};

static const TSFieldMapSlice ts_field_map_slices[10] = {
  [1] = {.index = 0, .length = 1},
  [2] = {.index = 1, .length = 1},
  [3] = {.index = 2, .length = 1},
  [4] = {.index = 3, .length = 2},
  [5] = {.index = 5, .length = 1},
  [6] = {.index = 6, .length = 2},
  [7] = {.index = 8, .length = 2},
  [8] = {.index = 10, .length = 1},
  [9] = {.index = 11, .length = 2},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_body, 0},
  [1] =
    {field_body, 0, .inherited = true},
  [2] =
    {field_expression, 0},
  [3] =
    {field_body, 0, .inherited = true},
    {field_body, 1, .inherited = true},
  [5] =
    {field_argument, 0},
  [6] =
    {field_argument, 0},
    {field_argument, 1, .inherited = true},
  [8] =
    {field_arguments, 2},
    {field_function, 0},
  [10] =
    {field_argument, 1},
  [11] =
    {field_argument, 0, .inherited = true},
    {field_argument, 1, .inherited = true},
};

static TSSymbol ts_alias_sequences[10][MAX_ALIAS_SEQUENCE_LENGTH] = {
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
      if (eof) ADVANCE(2);
      if (lookahead == '"') ADVANCE(1);
      if (lookahead == '#') ADVANCE(3);
      if (lookahead == '(') ADVANCE(7);
      if (lookahead == ')') ADVANCE(8);
      if (lookahead == ',') ADVANCE(9);
      if (lookahead == ';') ADVANCE(4);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(6);
      END_STATE();
    case 1:
      if (lookahead == '"') ADVANCE(5);
      if (lookahead != 0) ADVANCE(1);
      END_STATE();
    case 2:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 3:
      ACCEPT_TOKEN(sym_comment);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(3);
      END_STATE();
    case 4:
      ACCEPT_TOKEN(anon_sym_SEMI);
      END_STATE();
    case 5:
      ACCEPT_TOKEN(sym_string);
      END_STATE();
    case 6:
      ACCEPT_TOKEN(sym_identifier);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(6);
      END_STATE();
    case 7:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 8:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 9:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    default:
      return false;
  }
}

static TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 0},
  [3] = {.lex_state = 0},
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
  [15] = {.lex_state = 0},
  [16] = {.lex_state = 0},
  [17] = {.lex_state = 0},
  [18] = {.lex_state = 0},
};

static uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_comment] = ACTIONS(3),
    [anon_sym_SEMI] = ACTIONS(1),
    [sym_string] = ACTIONS(1),
    [sym_identifier] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
  },
  [1] = {
    [sym_source_file] = STATE(16),
    [sym_statement] = STATE(9),
    [sym__expression] = STATE(17),
    [sym__literal] = STATE(17),
    [sym_call] = STATE(17),
    [aux_sym_source_file_repeat1] = STATE(2),
    [ts_builtin_sym_end] = ACTIONS(5),
    [sym_comment] = ACTIONS(3),
    [sym_string] = ACTIONS(7),
    [sym_identifier] = ACTIONS(9),
  },
  [2] = {
    [sym_statement] = STATE(9),
    [sym__expression] = STATE(17),
    [sym__literal] = STATE(17),
    [sym_call] = STATE(17),
    [aux_sym_source_file_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(11),
    [sym_comment] = ACTIONS(3),
    [sym_string] = ACTIONS(7),
    [sym_identifier] = ACTIONS(9),
  },
  [3] = {
    [sym_statement] = STATE(9),
    [sym__expression] = STATE(17),
    [sym__literal] = STATE(17),
    [sym_call] = STATE(17),
    [aux_sym_source_file_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(13),
    [sym_comment] = ACTIONS(3),
    [sym_string] = ACTIONS(15),
    [sym_identifier] = ACTIONS(18),
  },
};

static uint16_t ts_small_parse_table[] = {
  [0] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      sym_identifier,
    ACTIONS(21), 1,
      sym_string,
    STATE(18), 1,
      sym_arguments,
    STATE(11), 3,
      sym__expression,
      sym__literal,
      sym_call,
  [18] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      sym_identifier,
    ACTIONS(23), 1,
      sym_string,
    ACTIONS(25), 1,
      anon_sym_RPAREN,
    STATE(15), 3,
      sym__expression,
      sym__literal,
      sym_call,
  [36] = 5,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      sym_identifier,
    ACTIONS(23), 1,
      sym_string,
    ACTIONS(27), 1,
      anon_sym_RPAREN,
    STATE(15), 3,
      sym__expression,
      sym__literal,
      sym_call,
  [54] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(9), 1,
      sym_identifier,
    ACTIONS(23), 1,
      sym_string,
    STATE(15), 3,
      sym__expression,
      sym__literal,
      sym_call,
  [69] = 3,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(31), 1,
      anon_sym_LPAREN,
    ACTIONS(29), 3,
      anon_sym_SEMI,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [81] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(33), 3,
      ts_builtin_sym_end,
      sym_string,
      sym_identifier,
  [90] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(35), 3,
      ts_builtin_sym_end,
      sym_string,
      sym_identifier,
  [99] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(37), 1,
      anon_sym_RPAREN,
    ACTIONS(39), 1,
      anon_sym_COMMA,
    STATE(12), 1,
      aux_sym_arguments_repeat1,
  [112] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(41), 1,
      anon_sym_RPAREN,
    ACTIONS(43), 1,
      anon_sym_COMMA,
    STATE(14), 1,
      aux_sym_arguments_repeat1,
  [125] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(45), 3,
      anon_sym_SEMI,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [134] = 4,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(47), 1,
      anon_sym_RPAREN,
    ACTIONS(49), 1,
      anon_sym_COMMA,
    STATE(14), 1,
      aux_sym_arguments_repeat1,
  [147] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(52), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [155] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(54), 1,
      ts_builtin_sym_end,
  [162] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(56), 1,
      anon_sym_SEMI,
  [169] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(58), 1,
      anon_sym_RPAREN,
};

static uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(4)] = 0,
  [SMALL_STATE(5)] = 18,
  [SMALL_STATE(6)] = 36,
  [SMALL_STATE(7)] = 54,
  [SMALL_STATE(8)] = 69,
  [SMALL_STATE(9)] = 81,
  [SMALL_STATE(10)] = 90,
  [SMALL_STATE(11)] = 99,
  [SMALL_STATE(12)] = 112,
  [SMALL_STATE(13)] = 125,
  [SMALL_STATE(14)] = 134,
  [SMALL_STATE(15)] = 147,
  [SMALL_STATE(16)] = 155,
  [SMALL_STATE(17)] = 162,
  [SMALL_STATE(18)] = 169,
};

static TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [11] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1, .production_id = 2),
  [13] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, .production_id = 4),
  [15] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, .production_id = 4), SHIFT_REPEAT(17),
  [18] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, .production_id = 4), SHIFT_REPEAT(8),
  [21] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [23] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [25] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_arguments, 2, .production_id = 5),
  [27] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_arguments, 3, .production_id = 6),
  [29] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__expression, 1),
  [31] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [33] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 1, .production_id = 1),
  [35] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_statement, 2, .production_id = 3),
  [37] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_arguments, 1, .production_id = 5),
  [39] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [41] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_arguments, 2, .production_id = 6),
  [43] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [45] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_call, 4, .production_id = 7),
  [47] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_arguments_repeat1, 2, .production_id = 9),
  [49] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_arguments_repeat1, 2, .production_id = 9), SHIFT_REPEAT(7),
  [52] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_arguments_repeat1, 2, .production_id = 8),
  [54] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [56] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [58] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
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
