# Statement Structure
| Kind | Components | Status |
| - | - | :-: |
| Ident | Identifier | ✓ |
| NumLit | Number | ✓ |
| BinaryOp | Expr, Op, Expr | ✓ |
| UnaryOp | Op, Expr | ✓ |
| Scope | LeftBrace, [Stmt], RightBrace | ✓ |
| Group | LeftParen, Expr, RightParen |
| List&lt;T&gt; | [T, Comma(skip last)] | ✓ |
| IfStmt | If, Expr(bool), Scope | ✓ |
| WhileLoop | While, Expr(bool), Scope | ✓ |
| DoWhileLoop | Do, Scope, While, Expr(bool) | ✓ |
| Decl | Let, Ident, Assignment, Expr | ✓ |
| Decl | Let, Ident, Colon, Ident, Assignment, Expr | ✓ |
| Assign | Ident, Assignment, Expr |
| OpAssign | Ident, Op, Assignment, Expr |
| Param | Ident, Colon, Ident | ✓ |
| Function | Func, LeftParen, List&lt;Param&gt;, RightParen, Scope | 
| Function | Func, LeftParen, List&lt;Param&gt;, RightParen, Colon, List&lt;Ident&gt;, Scope |
| FuncCall | Ident, LeftParen, List&lt;Expr&gt; RightParen | 
| Tuple | LeftParen, List&lt;Expr&gt;, RightParen |