using cslox.Expr;
using cslox.Stmt;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace cslox
{
	internal class Resolver : IExprVisitor<object?>, Stmt.IStmtVisitor<object?>
	{
		private Interpreter interpreter;
		private Stack<Dictionary<string, bool>> scopes = new();
		private FunctionType currentFunction = FunctionType.None;
		private ClassType currentClass = ClassType.None;

		internal Resolver(Interpreter interpreter)
		{
			this.interpreter = interpreter;
		}

		public void Resolve(IList<IStmt> statements)
		{
			foreach (IStmt statement in statements)
			{
				Resolve(statement);
			}
		}

		public void Resolve(IStmt statement)
		{
			statement.Accept(this);
		}

		public void Resolve(IExpr expression)
		{
			expression.Accept(this);
		}

		void BeginScope()
		{
			scopes.Push(new Dictionary<string, bool>());
		}

		void EndScope()
		{
			scopes.Pop();
		}

		void Declare(Token name)
		{
			if (scopes.Count == 0) return;

			var scope = scopes.Peek();
			scope[name.lexeme] = false;
		}

		void Define(Token name)
		{
			if (scopes.Count == 0) return;

			var scope = scopes.Peek();
			scope[name.lexeme] = true;
		}

		void ResolveLocal(IExpr variable, Token name)
		{
			for (int i = 0; i < scopes.Count; ++i)
			{
				if (scopes.ElementAt(i).ContainsKey(name.lexeme))
				{
					interpreter.Resolve(variable, i);
					return;
				}
			}
		}

		void ResolveFunction(Function function, FunctionType type)
		{
			var enclosingType = currentFunction;
			currentFunction = type;
			BeginScope();
			foreach (Token param in function.parameters)
			{
				Declare(param);
				Define(param);
			}


			foreach (IStmt statement in (function.body as Block).statements)
			{
				Resolve(statement);
			}
			EndScope();
			currentFunction = enclosingType;
		}

		public object? VisitAssignExpr(Assign assign)
		{
			Resolve(assign.value);
			ResolveLocal(assign, assign.name);
			return null;
		}

		public object? VisitBinaryExpr(Binary binary)
		{
			Resolve(binary.left);
			Resolve(binary.right);

			return null;
		}

		public object? VisitBlockStmt(Block block)
		{
			BeginScope();
			Resolve(block.statements);
			EndScope();
			return null;
		}

		public object? VisitBreakStmtStmt(BreakStmt breakstmt)
		{
			return null;
		}

		public object? VisitCallExpr(Call call)
		{
			Resolve(call.callee);

			foreach (var arg in call.arguments)
			{
				Resolve(arg);
			}

			return null;
		}

		public object? VisitContinueStmtStmt(ContinueStmt continuestmt)
		{
			return null;
		}

		public object? VisitExpressionStmt(Expression expression)
		{
			Resolve(expression.expression);
			return null;
		}

		public object? VisitFunctionStmt(Function function)
		{
			Declare(function.name);
			Define(function.name);

			ResolveFunction(function, FunctionType.Function);

			return null;
		}

		public object? VisitGroupingExpr(Grouping grouping)
		{
			Resolve(grouping.expression);
			return null;
		}

		public object? VisitIfStmtStmt(IfStmt ifstmt)
		{
			Resolve(ifstmt.condition);
			Resolve(ifstmt.then);
			if (ifstmt.elseDo != null) Resolve(ifstmt.elseDo);

			return null;
		}

		public object? VisitLiteralExpr(Literal literal)
		{
			return null;
		}

		public object? VisitLogicalExpr(Logical logical)
		{
			Resolve(logical.left);
			Resolve(logical.right);
			return null;
		}

		public object? VisitReturnStmtStmt(ReturnStmt returnstmt)
		{
			if (currentFunction == FunctionType.None)
			{
				Program.Error(returnstmt.keyword.line, "Can't return from outside a function.");
			}

			if (returnstmt.toReturn != null)
			{
				if (currentFunction == FunctionType.Initializer)
				{
					Program.Error(returnstmt.keyword.line, "Can't return a value from an initializer.");
				}
				Resolve(returnstmt.toReturn);
			}

			return null;
		}

		public object? VisitUnaryExpr(Unary unary)
		{
			Resolve(unary.right);

			return null;
		}

		public object? VisitVariableExpr(Variable variable)
		{
			ResolveLocal(variable, variable.name);
			return null;
		}

		public object? VisitVarMutStmt(VarMut varmut)
		{
			Declare(varmut.name);
			if (varmut.initializer != null)
			{
				Resolve(varmut.initializer);
			}

			Define(varmut.name);
			return null;
		}

		public object? VisitVarStmt(Var var)
		{
			Declare(var.name);
			if (var.initializer != null)
			{
				Resolve(var.initializer);
			}

			Define(var.name);
			return null;
		}

		public object? VisitWhileStmtStmt(WhileStmt whilestmt)
		{
			return null;
		}

		public object? VisitClassStmt(Class classStmt)
		{
			ClassType enclosing = currentClass;
			currentClass = ClassType.Class;

			Declare(classStmt.name);
			Define(classStmt.name);

			if (classStmt.superclass != null)
			{
				if (classStmt.name.lexeme == classStmt.superclass.name.lexeme)
				{
					Program.Error(classStmt.superclass.name.line, "A class cannot inherit from itself.");
				}
				currentClass = ClassType.Subclass;
				Resolve(classStmt.superclass);
			}

			if (classStmt.superclass != null)
			{
				BeginScope();
				scopes.Peek()["super"] = true;
			}


			BeginScope();
			scopes.Peek()["self"] = true;

			foreach (Function method in classStmt.methods)
			{
				var type = FunctionType.Method;
				if (method.name.lexeme == "self") type = FunctionType.Initializer;
				ResolveFunction(method, type);
			}
			EndScope();

			if (classStmt.superclass != null)
			{
				EndScope();
			}
			currentClass = enclosing;

			return null;
		}

		public object? VisitGetExpr(Get get)
		{
			Resolve(get.obj);
			return null;
		}

		public object? VisitSetExpr(Set set)
		{
			Resolve(set.value);
			Resolve(set.obj);
			return null;
		}

		public object? VisitSelfExpr(Self self)
		{
			if (currentClass == ClassType.None)
			{
				Program.Error(self.keyword.line, "Can't use 'self' outside of a class.");
				return null;
			}
			ResolveLocal(self, self.keyword);
			return null;
		}

		public object? VisitSuperExpr(Super super)
		{
			if (currentClass == ClassType.None)
			{
				Program.Error(super.keyword.line, "Can't use 'super' outside of a class.");
			}
			else if (currentClass != ClassType.Subclass)
			{
				Program.Error(super.keyword.line, "Can't use 'super' in a class without a subclass.");
			}
			ResolveLocal(super, super.keyword);
			return null;
		}
	}
}
