struct Expr {}
struct Binary {
		left: Expr,
		operator: Token,
		right: Expr,
}
struct Grouping {
		expression: Expr,
}
struct Literal {
		value: String,
}
struct Unary {
		operator: Token,
		right: Expr,
}
