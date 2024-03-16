# Statement Structure
| Kind | Components | Parsable | Checkable |
| - | - | :-: | :-: |
| Ident | Identifier | ✓ |
| NumLit | Number | ✓ | ✓ |
| BinaryOp | Expr, Op, Expr | ✓ | ✓ |
| UnaryOp | Op, Expr | ✓ | ✓ |
| Block | LeftBrace, Stmt[], RightBrace | ✓ | ✓ |
| Group | LeftParen, Expr, RightParen | ✓ | ✓ |
| List&lt;T&gt; | [T, Comma(skip last)] | ✓ |
| IfStmt | If, Expr(bool), Block | ✓ | ✓ |
| WhileLoop | While, Expr(bool), Block | ✓ | ✓ |
| DoWhileLoop | Do, Block, While, Expr(bool) | ✓ | ✓ |
| Break | Break |
| Break | Break, Label |
| Continue | Continue | ✓ | ✓ |
| Return | Return | ✓ |
| Return | Return, Expr | ✓ |
| Decl | Let, Ident, Assignment, Expr | ✓ | ✓ |
| Decl | Let, Ident, Colon, Ident, Assignment, Expr | ✓ | ✓ |
| Assign | Ident, Assignment, Expr |
| OpAssign | Ident, Op, Assignment, Expr |
| Param | Ident, Colon, Ident | ✓ |
| Function | Func, LeftParen, List&lt;Param&gt;, RightParen, Block | ✓ |
| Function | Func, LeftParen, List&lt;Param&gt;, RightParen, Colon, List&lt;Ident&gt;, Block | ✓ |
| FuncCall | Ident, LeftParen, List&lt;Expr&gt; RightParen | 
| Tuple | LeftParen, List&lt;Expr&gt;, RightParen |
| Array | LeftBracket, List&lt;Expr&gt;, RightBracket |