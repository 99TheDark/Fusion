# Statement Structure
|Kind|Components|
|-|-|
| BinaryOp | Expr, Op, Expr |
| UnaryOp | Op, Expr |
| Condition | Expr, Comp, Expr |
| Scope | Expr(bool), LeftBrace, [Stmt], RightBrace |
| Group | LeftParen, Expr, RightParen |
| List&lt;T&gt; | [T, Comma(skip last)] |
| IfStmt | If, Expr(bool), Scope |
| WhileLoop | While, Expr(bool), Scope |
| DoWhileLoop | Do, Scope, While, Expr(bool) |
| Decl | Let, Ident, Assignment, Expr |
| Decl | Let, Ident, Colon, Ident, Assignment, Expr |
| Assign | Ident, Assignment, Expr |
| OpAssign | Ident, Op, Assignment, Expr |
| Param | Ident, Colon, Ident | 
| Function | Func, LeftParen, List&lt;Param&gt;, RightParen, Scope |
| Function | Func, LeftParen, List&lt;Param&gt;, RightParen, Colon, List&lt;Ident&gt;, Scope |