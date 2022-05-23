using System.Collections.Generic;
using System;
using cslox.Expr;

namespace cslox.Stmt
{
	internal interface IStmt
	{
		public R Accept<R>(IStmtVisitor<R> visitor);
	}
	internal interface IStmtVisitor<R>
	{
		R VisitBlockStmt(Block block);
		R VisitExpressionStmt(Expression expression);
		R VisitPrintStmt(Print print);
		R VisitVarStmt(Var var);
		R VisitVarMutStmt(VarMut varmut);
	}
	internal class Block : IStmt
	{
		internal List<IStmt> statements;
		internal Block(List<IStmt> statements)
		{
			this.statements = statements;
		}
		public R Accept<R>(IStmtVisitor<R> visitor) => visitor.VisitBlockStmt(this);
	}
	internal class Expression : IStmt
	{
		internal IExpr expression;
		internal Expression(IExpr expression)
		{
			this.expression = expression;
		}
		public R Accept<R>(IStmtVisitor<R> visitor) => visitor.VisitExpressionStmt(this);
	}
	internal class Print : IStmt
	{
		internal IExpr expression;
		internal Print(IExpr expression)
		{
			this.expression = expression;
		}
		public R Accept<R>(IStmtVisitor<R> visitor) => visitor.VisitPrintStmt(this);
	}
	internal class Var : IStmt
	{
		internal Token name;
		internal IExpr initializer;
		internal Var(Token name, IExpr initializer)
		{
			this.name = name;
			this.initializer = initializer;
		}
		public R Accept<R>(IStmtVisitor<R> visitor) => visitor.VisitVarStmt(this);
	}
	internal class VarMut : IStmt
	{
		internal Token name;
		internal IExpr initializer;
		internal VarMut(Token name, IExpr initializer)
		{
			this.name = name;
			this.initializer = initializer;
		}
		public R Accept<R>(IStmtVisitor<R> visitor) => visitor.VisitVarMutStmt(this);
	}
}
