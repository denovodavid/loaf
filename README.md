# Loaf programming language.

Maybe lets try a simple math expression first:

https://createlang.rs/01_calculator/grammar_lexer_parser.html

```
Program = _{ SOI ~ Expr ~ EOF }

Expr = { UnaryExpr | BinaryExpr }

Term = _{Int | "(" ~ Expr ~ ")" }

UnaryExpr = { Operator ~ Term }

BinaryExpr = { Term ~ (Operator ~ Term)* }

Operator = { "+" | "-" }

Int = @{ Operator? ~ ASCII_DIGIT+ }

WHITESPACE = _{ " " | "\t" }

EOF = _{ EOI | ";" }
```

```
Expr: + Int
    | + (Expr)
    | - Int
    | - (Expr)
    | Int (+ Int)* // Int || Int + Int + Int + Int
    | Int + (Expr)
    | Int - Int
    | Int - (Expr)
    | (Expr) + Int
    | (Expr) + (Expr)
    | (Expr) - Int
    | (Expr) - (Expr)
```

Better thing for precendence. So we can differentiate between non-terminals E and terminals T:
```
E -> T
   | T + E
   | T - E
T -> Int
   | Int * T
   | Int / T
   | (E)
   | -T

===============

ε = nothing more to add

E -> T -> + E
        | - E
        | ε
T -> Int -> * T
          | / T
          | ε
   | (E)
   | -T
```

Example:

```
fn main() {
    // stuff goes here
}
```

Some nearly Backus-Naur Form (BNF) nonsense:

```
Program = Function
Function = "fn" ID ( [Formal ⟦, Formal⟧*] ) : TYPE { ⟦Expr ;⟧* [Expr] }
Formal = ID : TYPE
Expr = "let" ID : TYPE = Expr
    | ID = Expr
    | ID ( [Expr ⟦, Expr⟧*] )
    | { ⟦Expr ;⟧* [Expr] }
    | ( Expr )
    | Expr + Expr
    | Expr - Expr
    | Expr * Expr
    | Expr / Expr
    | ID
    | INTEGER
    | STRING
    | "true"
    | "false"
```
