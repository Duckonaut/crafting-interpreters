using cslox.Expr;
using cslox.Stmt;
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

		internal List<IStmt> Parse()
		{
			List<IStmt> statements =  new List<IStmt>();
			while (!IsAtEnd())
			{
				statements.Add(Declaration());
			}
			return statements;
		}

		private IStmt Declaration()
		{
			try
			{
				if (Match(Token.TokenType.VAR)) return VarDeclaration();

				return Statement();
			}
			catch (ParseError e)
			{
				Synchronize();
				return null;
			}
		}

		private IStmt VarDeclaration()
		{
			bool mutable = false;

			if (Match(Token.TokenType.MUT)) mutable = true;

			Token name = Consume(Token.TokenType.IDENTIFIER, "Expected variable name.");

			IExpr initializer = null;

			if (Match(Token.TokenType.EQUAL))
			{
				initializer = Expression();
			}

			Consume(Token.TokenType.SEMICOLON, "Expected ';' after variable declaration.");

			if (mutable)
			{
				return new VarMut(name, initializer);
			}
			return new Var(name, initializer);
		}

		private IStmt Statement()
		{
			if (Match(Token.TokenType.PRINT))
			{
				return PrintStatement();
			}

			if (Match(Token.TokenType.LEFT_BRACE)) return BlockStatement();

			return ExpressionStatement();
		}

		private IStmt ExpressionStatement()
		{
			IExpr value = Expression();
			Consume(Token.TokenType.SEMICOLON, "Expected ';' after statement.");
			return new Expression(value);
		}

		private IStmt PrintStatement()
		{
			IExpr value = Expression();
			Consume(Token.TokenType.SEMICOLON, "Expected ';' after statement.");
			return new Print(value);
		}

		private IStmt BlockStatement()
		{
			List<IStmt> statements = new List<IStmt>();

			while (!Check(Token.TokenType.RIGHT_BRACE) && !IsAtEnd())
			{
				statements.Add(Declaration());
			}

			Consume(Token.TokenType.RIGHT_BRACE, "Expected a closing '}'");
			return new Block(statements);
		}

		private IExpr Expression()
		{
			return Assignment();
		}

		private IExpr Assignment()
		{
			IExpr expr = Equality();

			if (Match(Token.TokenType.EQUAL))
			{
				Token equals = Previous();
				IExpr value = Assignment();

				if (expr is Variable v)
				{
					Token name = v.name;
					return new Assign(name, value);
				}

				Error(equals, "Invalid assignment target.");
			}

			return expr;
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

			if (Match(Token.TokenType.IDENTIFIER))
			{
				return new Variable(Previous());
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
		internal Token token;
		internal string message;

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
