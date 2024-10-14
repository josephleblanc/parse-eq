# parse-eq
A library for parsing equations into functions

Need to:
- [x]  Write enum for tokens
- [x]  Turn string into a binary tree composed of elements
- [x]  Implement a binary tree capable of changes through mutable references.
- [x]  Implement in-order traversal for binary tree

## Token
A `Token` is the internal representation of all elements of the tree that
represents the equation. The `Token` is an `enum`, some members of which are new-type containers, e.g. `Token::Op(Operator::Add)`.

Below is a list of all elements that should be implemented:
### Token::Op(Operator)
- [x] Multiply
- [x] Divide
- [x] Add
- [x] Subtract
- [ ] Exponent
- [ ] Logarithm
- ...
### Token::UnOp(UnaryOperator)
- [x] Negation
- [x] Sine
- [x] Cosine
- [x] Tangent
- [ ] Cosecant
- [ ] Secant
- [ ] Cotangent
- [ ] ArcSine
- [ ] ArcCosine
- [ ] ArcTangent
- ...
### Token::Number(f32)
Numbers are currently handled as type `f32`, but it may be more appropriate to
treat them as rational numbers, constant irrational numbers (e.g. 'e', 'pi'),
and f32 types. There is some concern that the f32 could lead to a lack of
precision, and it is often more useful to have numbers represented as fractions
as opposed to decimals.
- [ ] Restructure `Number(f32)` into something like `Number(Num::Rational)`,
where `Num` is an enum with enough members to cover all use cases, e.g.
`Num::Irrational`, `Num::Transcendental`, and similar.
### Token::Var(Variable)
- [ ]

## lexer
The lexer turns a string into a `Vec` of tokens, e.g.

```rust
    let add = vec![Number(1.0), Op(Add), Number(2.0)];
    assert_eq!(adding_string, Lexer::new_inorder("1+2").unwrap().list);
```

Some edge cases may require modifications to current implementation to work correctly. The last such modification was for handling the `Negation` unary operator.
