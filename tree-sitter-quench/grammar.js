module.exports = grammar({
  name: 'quench',

  extras: $ => [/\s/, $.comment],

  rules: {
    source_file: $ => repeat(field('body', $.statement)),

    comment: $ => /#.*/,

    statement: $ => seq(field('expression', $._expression), ';'),

    _expression: $ => choice($._literal, $.identifier, $.call),

    _literal: $ => choice($.string),

    string: $ => /"[^"]*"/,

    identifier: $ => /\w+/,

    call: $ => seq(
      field('function', $.identifier),
      '(',
      field('arguments', $.arguments),
      ')',
    ),

    arguments: $ => seq(
      field('argument', $._expression),
      repeat(seq(',', field('argument', $._expression))),
      optional(','),
    ),
  },
});
