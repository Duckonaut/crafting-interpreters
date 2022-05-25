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
		R VisitIfStmtStmt(IfStmt ifstmt);
		R VisitPrintStmt(Print print);
		R VisitVarStmt(Var var);
		R VisitVarMutStmt(VarMut varmut);
		R VisitWhileStmtStmt(WhileStmt whilestmt);
		R VisitBreakStmtStmt(BreakStmt breakstmt);
		R VisitContinueStmtStmt(ContinueStmt continuestmt);
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
	internal class IfStmt : IStmt
	{
		internal IExpr condition;
		internal IStmt then;
		internal IStmt elseDo;
		internal IfStmt(IExpr condition, IStmt then, IStmt elseDo)
		{
			this.condition = condition;
			this.then = then;
			this.elseDo = elseDo;
		}
		public R Accept<R>(IStmtVisitor<R> visitor) => visitor.VisitIfStmtStmt(this);
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
	internal class WhileStmt : IStmt
	{
		internal IExpr condition;
		internal IStmt then;
		internal WhileStmt(IExpr condition, IStmt then)
		{
			this.condition = condition;
			this.then = then;
		}
		public R Accept<R>(IStmtVisitor<R> visitor) => visitor.VisitWhileStmtStmt(this);
	}
	internal class BreakStmt : IStmt
	{
		internal BreakStmt()
		{
		}
		public R Accept<R>(IStmtVisitor<R> visitor) => visitor.VisitBreakStmtStmt(this);
	}
	internal class ContinueStmt : IStmt
	{
		internal ContinueStmt()
		{
		}
		public R Accept<R>(IStmtVisitor<R> visitor) => visitor.VisitContinueStmtStmt(this);
	}
}
