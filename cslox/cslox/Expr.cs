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
		R VisitCallExpr(Call call);
		R VisitGetExpr(Get get);
		R VisitGroupingExpr(Grouping grouping);
		R VisitLiteralExpr(Literal literal);
		R VisitLogicalExpr(Logical logical);
		R VisitSelfExpr(Self self);
		R VisitSuperExpr(Super super);
		R VisitSetExpr(Set set);
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
	internal class Call : IExpr
	{
		internal IExpr callee;
		internal Token paren;
		internal List<IExpr> arguments;
		internal Call(IExpr callee, Token paren, List<IExpr> arguments)
		{
			this.callee = callee;
			this.paren = paren;
			this.arguments = arguments;
		}
		public R Accept<R>(IExprVisitor<R> visitor) => visitor.VisitCallExpr(this);
	}
	internal class Get : IExpr
	{
		internal IExpr obj;
		internal Token name;
		internal Get(IExpr obj, Token name)
		{
			this.obj = obj;
			this.name = name;
		}
		public R Accept<R>(IExprVisitor<R> visitor) => visitor.VisitGetExpr(this);
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
	internal class Logical : IExpr
	{
		internal IExpr left;
		internal Token op;
		internal IExpr right;
		internal Logical(IExpr left, Token op, IExpr right)
		{
			this.left = left;
			this.op = op;
			this.right = right;
		}
		public R Accept<R>(IExprVisitor<R> visitor) => visitor.VisitLogicalExpr(this);
	}
	internal class Self : IExpr
	{
		internal Token keyword;
		internal Self(Token keyword)
		{
			this.keyword = keyword;
		}
		public R Accept<R>(IExprVisitor<R> visitor) => visitor.VisitSelfExpr(this);
	}
	internal class Super : IExpr
	{
		internal Token keyword;
		internal Token method;
		internal Super(Token keyword, Token method)
		{
			this.keyword = keyword;
			this.method = method;
		}
		public R Accept<R>(IExprVisitor<R> visitor) => visitor.VisitSuperExpr(this);
	}
	internal class Set : IExpr
	{
		internal IExpr obj;
		internal Token name;
		internal IExpr value;
		internal Set(IExpr obj, Token name, IExpr value)
		{
			this.obj = obj;
			this.name = name;
			this.value = value;
		}
		public R Accept<R>(IExprVisitor<R> visitor) => visitor.VisitSetExpr(this);
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
