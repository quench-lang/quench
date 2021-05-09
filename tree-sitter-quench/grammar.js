module.exports = grammar({
  name: 'quench',

  extras: $ => [/\s/, $.comment],

  rules: {
    source_file: $ => repeat(field('declaration', $.declaration)),

    comment: $ => /#.*/,

    declaration: $ => seq(
      field('name', $.identifier),
      ':=',
      field('value', $._expression),
      ';',
    ),

    statement: $ => seq(field('expression', $._expression), ';'),

    _expression: $ => choice(
      $.parenthesized,
      $._literal,
      $.identifier,
      $.block,
      $.call,
      $.function,
    ),

    parenthesized: $ => seq('(', field('expression', $._expression), ')'),

    _literal: $ => choice(
      'null',
      $.boolean,
      $.integer,
      $.string,
      $.symbol,
      $.list,
      $.map,
    ),

    boolean: $ => choice('true', 'false'),

    integer: $ => /-?\d+/,

    string: $ => /"[^"]*"/,

    symbol: $ => /\.\w+/,

    list: $ => seq(
      '[',
      repeat(seq(field('item', $._expression), ',')),
      optional(field('item', $._expression)),
      ']',
    ),

    map: $ => seq(
      '{',
      repeat(seq(field('entry', $.entry), ',')),
      optional(field('entry', $.entry)),
      '}',
    ),

    entry: $ => seq(
      field('key', $._expression),
      ':',
      field('value', $._expression),
    ),

    identifier: $ => /\w+/,

    block: $ => seq(
      '{',
      repeat1(field('statement', $.statement)),
      optional(field('expression', $._expression)),
      '}',
    ),

    call: $ => prec(1, prec.left(seq(
      field('function', $._expression),
      field('argument', $._expression),
    ))),

    function: $ => seq(
      field('parameter', $.identifier),
      '=>',
      field('body', $._expression),
    ),
  },
});
