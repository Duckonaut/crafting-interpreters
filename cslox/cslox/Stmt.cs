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
		R VisitClassStmt(Class cl);
		R VisitExpressionStmt(Expression expression);
		R VisitFunctionStmt(Function function);
		R VisitIfStmtStmt(IfStmt ifstmt);
		R VisitVarStmt(Var var);
		R VisitVarMutStmt(VarMut varmut);
		R VisitWhileStmtStmt(WhileStmt whilestmt);
		R VisitReturnStmtStmt(ReturnStmt returnstmt);
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
	internal class Class : IStmt
	{
		internal Token name;
		internal Variable superclass;
		internal List<Function> methods;
		internal Class(Token name, Variable superclass, List<Function> methods)
		{
			this.name = name;
			this.superclass = superclass;
			this.methods = methods;
		}
		public R Accept<R>(IStmtVisitor<R> visitor) => visitor.VisitClassStmt(this);
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
	internal class Function : IStmt
	{
		internal Token name;
		internal List<Token> parameters;
		internal IStmt body;
		internal Function(Token name, List<Token> parameters, IStmt body)
		{
			this.name = name;
			this.parameters = parameters;
			this.body = body;
		}
		public R Accept<R>(IStmtVisitor<R> visitor) => visitor.VisitFunctionStmt(this);
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
	internal class ReturnStmt : IStmt
	{
		internal Token keyword;
		internal IExpr? toReturn;
		internal ReturnStmt(Token keyword, IExpr? toReturn)
		{
			this.keyword = keyword;
			this.toReturn = toReturn;
		}
		public R Accept<R>(IStmtVisitor<R> visitor) => visitor.VisitReturnStmtStmt(this);
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
