module.exports = grammar({
  name: 'quench',

  extras: $ => [/\s/, $.comment],

  rules: {
    source_file: $ => repeat($._statement),

    comment: $ => /#.*/,

    _statement: $ => seq(choice($._expression), ';'),

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
      $._expression,
      repeat(seq(',', $._expression)),
      optional(','),
    ),
  },
});
