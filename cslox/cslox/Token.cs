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
			AND, CLASS, ELSE, FALSE, FN, FOR, IF, NIL, OR,
			RETURN, SUPER, SELF, TRUE, VAR, WHILE, MUT, BREAK, CONTINUE,

			EOF,
			BUILTIN
		}

		internal static Dictionary<string, TokenType> Keywords = new Dictionary<string, TokenType>() {
			{ "and", TokenType.AND },
			{ "class", TokenType.CLASS },
			{ "else", TokenType.ELSE },
			{ "false", TokenType.FALSE },
			{ "fn", TokenType.FN },
			{ "for", TokenType.FOR },
			{ "if", TokenType.IF },
			{ "nil", TokenType.NIL },
			{ "or", TokenType.OR },
			{ "return", TokenType.RETURN },
			{ "super", TokenType.SUPER },
			{ "self", TokenType.SELF },
			{ "true", TokenType.TRUE },
			{ "var", TokenType.VAR },
			{ "while", TokenType.WHILE },
			{ "mut", TokenType.MUT },
			{ "break", TokenType.BREAK },
			{ "continue", TokenType.CONTINUE },
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
