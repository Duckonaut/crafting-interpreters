using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace cslox
{
	internal class Token
	{
		internal enum TokenType
		{
			LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
			COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

			// One or two character tokens.
			BANG, BANG_EQUAL,
			EQUAL, EQUAL_EQUAL,
			GREATER, GREATER_EQUAL,
			LESS, LESS_EQUAL,

			// Literals.
			IDENTIFIER, STRING, NUMBER,

			// Keywords.
			AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
			PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

			EOF
		}

		internal static Dictionary<string, TokenType> Keywords = new Dictionary<string, TokenType>() {
			{ "and", TokenType.AND },
			{ "class", TokenType.CLASS },
			{ "else", TokenType.ELSE },
			{ "false", TokenType.FALSE },
			{ "fun", TokenType.FUN },
			{ "for", TokenType.FOR },
			{ "if", TokenType.IF },
			{ "nil", TokenType.NIL },
			{ "or", TokenType.OR },
			{ "print", TokenType.PRINT },
			{ "return", TokenType.RETURN },
			{ "super", TokenType.SUPER },
			{ "this", TokenType.THIS },
			{ "true", TokenType.TRUE },
			{ "var", TokenType.VAR },
			{ "while", TokenType.WHILE },
		};

		internal TokenType type;
		internal string lexeme;
		internal object? literal;
		internal int line;

		internal Token(TokenType type, string lexeme, object? literal, int line)
		{
			this.type = type;
			this.lexeme = lexeme;
			this.literal = literal;
			this.line = line;
		}

		public override string ToString()
		{
			return $"{type} {lexeme} {literal}";
		}
	}
}
