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
		R VisitAssignExpr(Assign assign);
		R VisitBinaryExpr(Binary binary);
		R VisitGroupingExpr(Grouping grouping);
		R VisitLiteralExpr(Literal literal);
		R VisitUnaryExpr(Unary unary);
		R VisitVariableExpr(Variable variable);
	}
	internal class Assign : IExpr
	{
		internal Token name;
		internal IExpr value;
		internal Assign(Token name, IExpr value)
		{
			this.name = name;
			this.value = value;
		}
		public R Accept<R>(IExprVisitor<R> visitor) => visitor.VisitAssignExpr(this);
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
	internal class Variable : IExpr
	{
		internal Token name;
		internal Variable(Token name)
		{
			this.name = name;
		}
		public R Accept<R>(IExprVisitor<R> visitor) => visitor.VisitVariableExpr(this);
	}
}
