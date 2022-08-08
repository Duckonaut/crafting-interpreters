using cslox.Expr;
using cslox.Stmt;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading.Tasks;

namespace cslox
{
	internal class Interpreter : IExprVisitor<object?>, IStmtVisitor<object?>
	{
		internal Environment globals = new Environment();
		private Environment env;
		private Dictionary<IExpr, int> locals = new();

		public Interpreter()
		{
			env = globals;
			globals.Define(new Token(Token.TokenType.BUILTIN, "clock", null, -1), new NativeLoxCallable(0, (i, args) =>
			{
				return (double)DateTimeOffset.Now.ToUnixTimeMilliseconds();
			}));

			globals.Define(new Token(Token.TokenType.BUILTIN, "print", null, -1), new NativeLoxCallable(1, (i, args) =>
			{
				Console.Write(args[0]);
				return null;
			}));

			globals.Define(new Token(Token.TokenType.BUILTIN, "println", null, -1), new NativeLoxCallable(1, (i, args) =>
			{
				Console.WriteLine(args[0]);
				return null;
			}));

			globals.Define(new Token(Token.TokenType.BUILTIN, "show", null, -1), new NativeLoxCallable(1, (i, args) =>
			{
				return args[0].ToString();
			}));
		}

		public void Interpret(List<IStmt> statements)
		{
			try
			{
				foreach (IStmt stmt in statements)
				{
					Execute(stmt);
				}
			}
			catch (RuntimeError ex)
			{
				Program.RuntimeError(ex);
			}
		}

		public object? Execute(IStmt stmt)
		{
			return stmt.Accept(this);
		}

		public object? ExecuteBlock(List<IStmt> statements, Environment newEnvironment)
		{
			Environment prev = env;
			object? value = null;
			try
			{
				env = newEnvironment;

				foreach (IStmt stmt in statements)
				{
					value = Execute(stmt);
				}
			}
			finally
			{
				env = prev;
			}

			return value;
		}

		private string Stringify(object? obj)
		{
			if (obj == null) return "nil";

			if (obj is string s)
			{
				return s;
			}


			return obj.ToString();
		}

		public object? VisitBinaryExpr(Binary binary)
		{
			object? left = Evaluate(binary.left);
			object? right = Evaluate(binary.right);

			switch (binary.op.type)
			{
				case Token.TokenType.MINUS:
					CheckNumberOperands(binary.op, left, right);
					return (double)left - (double)right;
				case Token.TokenType.PLUS:
					if (left is double d && right is double d2) return d + d2;
					if (left is string s1 && right is string s2) return s1 + s2;
					return new RuntimeError(binary.op, "Operands must be 2 numbers or 2 strings.");
					return (double)left + (double)right;
				case Token.TokenType.STAR:
					CheckNumberOperands(binary.op, left, right);
					return (double)left * (double)right;
				case Token.TokenType.SLASH:
					CheckNumberOperands(binary.op, left, right);
					return (double)left * (double)right;
				case Token.TokenType.GREATER:
					CheckNumberOperands(binary.op, left, right);
					return (double)left > (double)right;
				case Token.TokenType.GREATER_EQUAL:
					CheckNumberOperands(binary.op, left, right);
					return (double)left >= (double)right;
				case Token.TokenType.LESS:
					CheckNumberOperands(binary.op, left, right);
					return (double)left < (double)right;
				case Token.TokenType.LESS_EQUAL:
					CheckNumberOperands(binary.op, left, right);
					return (double)left <= (double)right;
				case Token.TokenType.BANG_EQUAL:
					return !IsEqual(left, right);
				case Token.TokenType.EQUAL_EQUAL:
					return IsEqual(left, right);
			}

			return null;
		}

		public object? VisitGroupingExpr(Grouping grouping)
		{
			return Evaluate(grouping.expression);
		}

		public object? VisitLiteralExpr(Literal literal)
		{
			return literal.value;
		}

		public object? VisitUnaryExpr(Unary unary)
		{
			object? right = Evaluate(unary.right);

			switch (unary.op.type)
			{
				case Token.TokenType.MINUS:
					return -(double)right;
				case Token.TokenType.BANG:
					return !Truthy(right);
			}

			return null;
		}

		public object? VisitCallExpr(Call call)
		{
			object? callee = Evaluate(call.callee);

			List<object?> arguments = new List<object?>();

			if (call.arguments != null)
			{
				foreach (var arg in call.arguments)
					arguments.Add(Evaluate(arg));
			}
			if (callee is ILoxCallable lc)
			{
				return lc.Call(this, arguments);
			}

			throw new RuntimeError(call.paren, "Callee is not a callable object!");
		}

		private object? Evaluate(IExpr expr)
		{
			return expr.Accept(this);
		}

		private bool Truthy(object? obj)
		{
			if (obj == null) return false;
			if (obj is bool b) return b;
			if (obj is double d) return d != 0;
			return true;
		}

		private bool IsEqual(object? obj1, object? obj2)
		{
			if (obj1 == null && obj2 == null) return true;
			if (obj2 == null) return false;

			return obj1.Equals(obj2);
		}

		private void CheckNumberOperand(Token op, object? operand)
		{
			if (operand is double) return;
			throw new RuntimeError(op, "Operand must be a number.");
		}

		private void CheckNumberOperands(Token op, params object?[] operands)
		{
			foreach (var operand in operands)
				CheckNumberOperand(op, operand);
		}

		public void Resolve(IExpr expr, int depth)
		{
			locals[expr] = depth;
		}

		public object? LookUpVariable(Token name, IExpr expr)
		{
			if (locals.ContainsKey(expr))
			{
				return env.GetAt(locals[expr], name.lexeme);
			}
			else
			{
				return globals.Get(name);
			}
		}

		public object? VisitExpressionStmt(Expression expression)
		{
			Evaluate(expression.expression);
			return null;
		}

		public object? VisitVarStmt(Var var)
		{
			object? value = null;
			if (var.initializer != null)
			{
				value = Evaluate(var.initializer);
			}

			env.Define(var.name, value);
			return null;
		}

		public object? VisitVariableExpr(Variable variable)
		{
			return LookUpVariable(variable.name, variable);
		}

		public object? VisitVarMutStmt(VarMut varmut)
		{
			object? value = null;
			if (varmut.initializer != null)
			{
				value = Evaluate(varmut.initializer);
			}

			env.Define(varmut.name, value, true);
			return null;
		}

		public object? VisitFunctionStmt(Function function)
		{
			LoxFunction loxFunction = new LoxFunction(function, env);
			env.Define(function.name, loxFunction);
			return env.Define(function.name, loxFunction);
		}

		public object? VisitClassStmt(Class stmt)
		{
			env.Define(stmt.name, null, true);
			LoxClass cl = new LoxClass(stmt.name.lexeme);
			env.Assign(stmt.name, cl);

			return null;
		}

		public object? VisitAssignExpr(Assign assign)
		{
			if (locals.ContainsKey(assign))
			{
				if (env.MutableAt(locals[assign], assign.name))
				{
					object? value = null;
					if (assign.value != null)
					{
						value = Evaluate(assign.value);
					}

					env.AssignAt(locals[assign], assign.name, value);
				}
				else
				{
					throw new RuntimeError(assign.name, "Attempted to mutate a variable not marked as 'mut'");
				}
			}
			else
			{
				if (globals.Mutable(assign.name))
				{
					object? value = null;
					if (assign.value != null)
					{
						value = Evaluate(assign.value);
					}

					globals.Assign(assign.name, value);
				}
				else
				{
					throw new RuntimeError(assign.name, "Attempted to mutate a variable not marked as 'mut'");
				}
			}

			return null;
		}

		public object? VisitBlockStmt(Block block)
		{
			return ExecuteBlock(block.statements, new Environment(env));
		}

		public object? VisitIfStmtStmt(IfStmt ifstmt)
		{
			if (Truthy(Evaluate(ifstmt.condition)))
			{
				return Execute(ifstmt.then);
			}
			else if (ifstmt.elseDo != null)
			{
				return Execute(ifstmt.elseDo);
			}

			return null;
		}

		public object? VisitLogicalExpr(Logical logical)
		{
			object? left = Evaluate(logical.left);

			if (logical.op.type == Token.TokenType.OR)
			{
				if (Truthy(left)) return left;
			}
			else
			{
				if (!Truthy(left)) return left;
			}

			return Evaluate(logical.right);
		}

		public object? VisitWhileStmtStmt(WhileStmt whilestmt)
		{
			while (Truthy(Evaluate(whilestmt.condition)))
			{
				try
				{
					Execute(whilestmt.then);
				}
				catch (LoopBreakException lb)
				{
					if (lb.type == LoopBreakException.LoopBreakType.Break) break;
					else continue;
				}
			}

			return null;
		}

		public object? VisitBreakStmtStmt(BreakStmt breakstmt)
		{
			throw new LoopBreakException(LoopBreakException.LoopBreakType.Break);
		}

		public object? VisitContinueStmtStmt(ContinueStmt continuestmt)
		{
			throw new LoopBreakException(LoopBreakException.LoopBreakType.Continue);
		}

		public object? VisitReturnStmtStmt(ReturnStmt returnstmt)
		{
			if (returnstmt.toReturn == null) return null;
			throw new ReturnException(Evaluate(returnstmt.toReturn));
		}

		public object? VisitGetExpr(Get get)
		{
			object? obj = Evaluate(get.obj);
			if (obj is LoxInstance li)
			{
				return li.Get(get.name);
			}

			throw new RuntimeError(get.name, "Only instances of classes can have properties.");
		}

		public object? VisitSetExpr(Set set)
		{
			object? obj = Evaluate(set.obj);

			if (obj is LoxInstance li)
			{
				object? value = Evaluate(set.value);
				li.Set(set.name, value);
				return value;
			}

			throw new RuntimeError(set.name, "Only instances of classes have fields.");
		}
	}

	internal class RuntimeError : Exception
	{
		internal Token token;

		internal RuntimeError(Token token, string message) : base(message)
		{
			this.token = token;
		}
	}
}
