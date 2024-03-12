# Statement Structure
|Kind|Components|Status|
|-|-|:-:|
| Ident | Identifier | ✓ |
| BinaryOp | Expr, Op, Expr | ✓ |
| UnaryOp | Op, Expr |
| Scope | LeftBrace, [Stmt], RightBrace | ✓ |
| Group | LeftParen, Expr, RightParen |
| List&lt;T&gt; | [T, Comma(skip last)] |
| IfStmt | If, Expr(bool), Scope | ✓ |
| WhileLoop | While, Expr(bool), Scope |
| DoWhileLoop | Do, Scope, While, Expr(bool) |
| Decl | Let, Identifier, Assignment, Expr |
| Decl | Let, Identifier, Colon, Identifier, Assignment, Expr |
| Assign | Identifier, Assignment, Expr |
| OpAssign | Identifier, Op, Assignment, Expr |
| Param | Identifier, Colon, Identifier | 
| Function | Func, LeftParen, List&lt;Param&gt;, RightParen, Scope |
| Function | Func, LeftParen, List&lt;Param&gt;, RightParen, Colon, List&lt;Identifier&gt;, Scope |
| Tuple | LeftParen, List&lt;Expr&gt;, RightParen |