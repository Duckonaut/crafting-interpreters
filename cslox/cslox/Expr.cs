using System.Collections.Generic;
using System;

namespace cslox.Expr
{
	internal interface IExpr
	{
		public R Accept<R>(IExprVisitor<R> visitor);
	}
	internal interface IExprVisitor<R>
	{
		R VisitBinaryExpr(Binary binary);
		R VisitGroupingExpr(Grouping grouping);
		R VisitLiteralExpr(Literal literal);
		R VisitUnaryExpr(Unary unary);
	}
	internal class Binary : IExpr
	{
		internal IExpr left;
		internal Token op;
		internal IExpr right;
		internal Binary(IExpr left, Token op, IExpr right)
		{
			this.left = left;
			this.op = op;
			this.right = right;
		}
		public R Accept<R>(IExprVisitor<R> visitor) => visitor.VisitBinaryExpr(this);
	}
	internal class Grouping : IExpr
	{
		internal IExpr expression;
		internal Grouping(IExpr expression)
		{
			this.expression = expression;
		}
		public R Accept<R>(IExprVisitor<R> visitor) => visitor.VisitGroupingExpr(this);
	}
	internal class Literal : IExpr
	{
		internal object? value;
		internal Literal(object? value)
		{
			this.value = value;
		}
		public R Accept<R>(IExprVisitor<R> visitor) => visitor.VisitLiteralExpr(this);
	}
	internal class Unary : IExpr
	{
		internal Token op;
		internal IExpr right;
		internal Unary(Token op, IExpr right)
		{
			this.op = op;
			this.right = right;
		}
		public R Accept<R>(IExprVisitor<R> visitor) => visitor.VisitUnaryExpr(this);
	}
}
