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
- [x] classic variables x, y, z
- [ ] other lower-case letters: a-w
- [ ] upper-case letters: A-Z
- [ ] greek letters
- [ ] sub-scripts
- [ ] super-scripts

## lexer
The lexer turns a string into a `Vec` of tokens, e.g.

```rust
    let add = vec![Number(1.0), Op(Add), Number(2.0)];
    assert_eq!(adding_string, Lexer::new_inorder("1+2").unwrap().list);
```

Some edge cases may require modifications to current implementation to work
correctly. The last such modification was for handling the `Negation` unary
operator.

- [ ] handle sub-scripts
- [ ] handle super-scripts

## tree
The binary tree contains the bulk of the functionality in this program.
Currently the binary tree can be created and saved to a typst file, which then
can represent the binary tree. However, much work is needed to fill out the
desired functionality of solving algebra equations, and `tree` will be the
module doing most of the heavy lifting.

### saving to typst
- [x] save binary tree graph representation to typst with
`some_tree.save_typst("filename.typ")`;
- [ ] save expression representation to typst with yet-to-be-named function.

### parsing
The tree is created from a list of tokens. The `Vec<Token>` is created from a
string by `lexer`, but `tree` handles the conversion into a binary expression
tree.
- [x] convert basic expressions without parentheses, e.g. (`1 + 2`)
- [x] handle parentheses
- [x] handle unary operators

### mutating the binary expression tree (BET)
The BET is the internal representation of a well-formed math expression, and is modified by mutating the BET in accordance with the rules of algebra. This requires that a large number of possible patterns be detected, and then modify the tree to make some change.

For example, consider the following tree of the expression `a * (b + c)`:
```
    *
   / \
  a   +
     / \
    b   c
```
One possible valid operation on this BET is to distribute, such that `a * (b +
c) => ab + ac`, with the corresponding binary tree being:
```
      +
    /   \
   /     \
  *       *
 / \     / \
a   b   a   c
```
In order to encode this algebraic rule into the possible valid operations on
the BET, we must first detect the pattern in the first tree above, where `a`,
`b`, and `c` are any nodes either in their own tree or a branch of another
tree, then mutate that branch to the second BET.

#### Concerns
There are many possible rules to include, and so it would be well worth looking
into a way to either compose rules or somehow reduce the requirement of
encoding each individual rule.

#### Rules to Implement
Where the following letters `a`, `b`, etc. are any possible nodes

**Basic identities**
*Negation*
[ ] `-(-a) -> a`
*Addition and Subtraction*
[ ] `a - a => 0`
[ ] `a + 0 => a`
[ ] `a - 0 => a`
[ ] `0 + a => a`
[ ] `0 - a => -a`
*Multiplication and Division*
[ ] `a / a => 1`
[ ] `-a / a => -1`
[ ] `a / -a => -1`
[ ] `-a / -a => 1`
[ ] `a * a => 1`
[ ] `-a * a => -1`
[ ] `a * -a => -1`
[ ] `-a * -a => 1`
[ ] `a * 1 => a`
[ ] `a * -1 => -a`
[ ] `a * 1 => -a`

**Relations**
*Multiplication distributive over addition & subtraction*
[ ] `a * (b + c) => ab + ac`
[ ] `a * (b - c) => ab - ac`
[ ] `(b + c) * a => ba + ca`
[ ] `(b - c) * a => ab - ac`

... more here
