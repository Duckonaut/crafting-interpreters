using cslox.Expr;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace cslox
{
	internal class Parser
	{
		private List<Token> tokens;
		private int current = 0;

		public Parser(List<Token> tokens)
		{
			this.tokens = tokens;
		}

		internal IExpr Parse()
		{
			try
			{
				return Expression();
			}
			catch (ParseError pe)
			{
				return null;
			}
		}

		private IExpr Expression()
		{
			return Equality();
		}

		private IExpr Equality()
		{
			IExpr expr = Comparison();

			while (Match(Token.TokenType.BANG_EQUAL, Token.TokenType.EQUAL_EQUAL))
			{
				Token op = Previous();
				IExpr right = Comparison();
				expr = new Binary(expr, op, right);
			}

			return expr;
		}

		private IExpr Comparison()
		{
			IExpr expr = Term();

			while (Match(Token.TokenType.GREATER, Token.TokenType.GREATER_EQUAL, Token.TokenType.LESS, Token.TokenType.LESS_EQUAL))
			{
				Token op = Previous();
				IExpr right = Term();
				expr = new Binary(expr, op, right);
			}

			return expr;
 		}

		private IExpr Term()
		{
			IExpr expr = Factor();

			while (Match(Token.TokenType.MINUS, Token.TokenType.PLUS))
			{
				Token op = Previous();
				IExpr right = Factor();
				expr = new Binary(expr, op, right);
			}

			return expr;
		}

		private IExpr Factor()
		{
			IExpr expr = Unary();

			while (Match(Token.TokenType.SLASH, Token.TokenType.STAR))
			{
				Token op = Previous();
				IExpr right = Unary();
				expr = new Binary(expr, op, right);
			}

			return expr;
		}

		private IExpr Unary()
		{
			if (Match(Token.TokenType.BANG, Token.TokenType.MINUS))
			{
				Token op = Previous();
				IExpr right = Unary();
				return new Unary(op, right);
			}

			return Primary();
		}

		private IExpr Primary()
		{
			if (Match(Token.TokenType.FALSE)) return new Literal(false);
			if (Match(Token.TokenType.TRUE)) return new Literal(true);
			if (Match(Token.TokenType.NIL)) return new Literal(null);

			if (Match(Token.TokenType.NUMBER, Token.TokenType.STRING))
			{
				return new Literal(Previous().literal);
			}

			if (Match(Token.TokenType.LEFT_PAREN))
			{
				IExpr expr = Expression();
				Consume(Token.TokenType.RIGHT_PAREN, "Expected ')' after the expression.");
				return new Grouping(expr);
			}

			throw Error(Peek(), "Expected expression.");
		}

		private bool Match(params Token.TokenType[] types)
		{
			foreach (var type in types)
			{
				if (Check(type))
				{
					Advance();
					return true;
				}
			}

			return false;
		}

		private Token Advance()
		{
			if (!IsAtEnd()) current++;
			return Previous();
		}

		private bool Check(Token.TokenType type)
		{
			if (IsAtEnd()) return false;
			return Peek().type == type;
		}

		private Token Consume(Token.TokenType type, string errMessage)
		{
			if (Check(type))
			{
				return Advance();
			}
			throw Error(Peek(), errMessage);
		}

		private ParseError Error(Token token, string message)
		{
			Program.Error(token.line, message);
			return new ParseError(token, message);
		}

		private void Synchronize()
		{
			Advance();

			while (!IsAtEnd())
			{
				if (Previous().type == Token.TokenType.SEMICOLON) return;

				switch (Peek().type)
				{
					case Token.TokenType.CLASS:
					case Token.TokenType.FUN:
					case Token.TokenType.VAR:
					case Token.TokenType.FOR:
					case Token.TokenType.IF:
					case Token.TokenType.WHILE:
					case Token.TokenType.PRINT:
					case Token.TokenType.RETURN:
						return;
				}
			}

			Advance();
		}

		private bool IsAtEnd()
		{
			return Peek().type == Token.TokenType.EOF;
		}

		private Token Peek()
		{
			return tokens[current];
		}

		private Token Previous()
		{
			return tokens[current - 1];
		}
	}

	internal class ParseError : Exception
	{
		Token token;
		string message;

		public ParseError(Token token, string message)
		{
			this.token = token;
			this.message = message;
		}

		public override string Message
		{
			get
			{
				if (token.type == Token.TokenType.EOF)
				{
					return $"{token.line} at end {message}";
				}
				else
				{
					return $"{token.line} at '{token.lexeme}' {message}";
				}
			}
		}
	}
}
