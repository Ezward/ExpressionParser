#Expression Parser

A simple expression parser and evaluator for the 4 operations and exponentiation.

Ported from the Java version [here](https://github.com/Ezward/ExpressionCalculator/tree/master/src/com/lumpofcode/expression/associative)

```
Usage:
  let s = " (((10 + 5) * -6) - -20.0 / -2 * 3 + -((5*2)^2) - (-5 * -2 * 5)) ";
  let (_result_context, result_node) = parse_expression(s, beginning()).unwrap();
  assert_eq!(result_node.evaluate(), ExpressionValue::Decimal { value: -270 as DecimalType});
```

Running from command line:
```
% cargo run " (((10 + 5) * -6) - -20.0 / -2 * 3 + -((5*2)^2) - (-5 * -2 * 5)) "
-270
```

NOTE: this grammar separates out sums from differences and products from quotients.
      Thus, it is not a traditional factor/term grammar.  The grammar is
      designed to separate out operations that are subject to the associative
      and commutative properties with the notion that the parse tree can
      then be more easily queried or manipulated using those mathematical properties.


<img width="605" height="1685" alt="image" src="https://github.com/user-attachments/assets/a83e83b7-d500-4fad-b000-81b415f94195" />


This is the equivalent PEG grammar:

```
digit ::= [0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9]
sign ::= '-'
integer ::= {sign} [digit]*
decimal ::= {sign} [digit]* '.' [digit]*
scientific ::= {sign} [digit]* {'.' [digit]*} ['e' | 'E'] {sign} [digit]*
number ::= [integer | decimal | scientific]
parenthesis ::= {sign} '(' expression ')'
value ::= [parenthesis | number]
power ::= value{'^'value}
quotient ::= power {['÷' | '/'] power}*
product ::= quotient {['×' | '*']  quotient}*
difference ::= product  {'-' product}*
sum ::= difference {'+' difference}*
expression ::= sum
```

Key to PEG notation:
```
{} = optional, choose zero or one
{}* = optional, 0 or more
[] = required, choose one
[]* = required, 1 or more
```

Here is an equivalent EBNF grammar:

```
digit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9";
unsigned_integer = digit, {digit};
sign = "-";
signed_integer = [sign], unsigned_integer;
decimal = [sign], digit, {digit}, '.', digit, {digit};
scientific = signed_integer, ['.', unsigned_integer], ('e' | 'E'), signed_integer
number = integer | decimal | scientific;
parenthesis = [sign], "(", expression, ")";
value = parenthesis | number;
power = value, ["^", value];
quotient = power, {('÷' | '/'), power};
product = quotient, {('×' | '*'),  quotient};
difference = product,  {"-", product};
sum = difference, {"+", difference};
expression = sum;
```

The EBNF notation is the notation used by [Platinum UML](https://plantuml.com/ebnf).
