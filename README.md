# Loaf programming language.

Maybe lets try a simple math expression first:

```
S -> E;*
E -> T
   | T + E
   | T - E
T -> Int
   | Int * T
   | Int / T
   | (E)
   | -T

===============

LL(1) implementation is essentially this:

SL -> S*
S -> E;
E -> T -> + E
        | - E
        | ε
T -> Int -> * T
          | / T
          | ε
   | ( E )
   | - T

(ε = nothing more to add)

===============

Left-derivded form:

L -> S*
S -> E ;
E -> T X
X -> + E
   | - E
   | ε
T -> Int Y
   | ( E )
   | - T
Y -> * T
   | / T
   | ε

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
