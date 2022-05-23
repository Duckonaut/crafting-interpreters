using cslox.Expr;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace cslox
{
	internal class Interpreter : IExprVisitor<object?>
	{
		public void Interpret(IExpr expression)
		{
			try
			{
				object? value = Evaluate(expression);
				Console.WriteLine(Stringify(value));
			}
			catch (RuntimeError ex)
			{
				Program.RuntimeError(ex);
			}
		}

		private string Stringify(object? obj)
		{
			if (obj == null) return "nil";

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
