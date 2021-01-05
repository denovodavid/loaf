# Loaf programming language.

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