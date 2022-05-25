using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading.Tasks;

namespace cslox
{
	internal class Lexer
	{

		private string source;
		private List<Token> tokens;
		private int start;
		private int current;
		private int line;
		private int lineStart;

		private readonly Regex newlineReplacerRegex = new Regex(@"(^|[^\\])(\\n)");
		private readonly Regex tabReplacerRegex = new Regex(@"(^|[^\\])(\\t)");
		private readonly Regex backslashReplacerRegex = new Regex(@"(\\\\)");

		internal Lexer(string source)
		{
			this.source = source;
			tokens = new List<Token>();
			start = 0;
			current = 0;
			line = 1;
			lineStart = 0;
		}

		internal List<Token> GetTokens()
		{
			while (!IsAtEnd())
			{
				start = current;
				GetToken();
			}

			tokens.Add(new Token(Token.TokenType.EOF, "", null, line));
			return tokens;
		}

		void GetToken()
		{
			char c = Advance();

			switch (c)
			{
				case '(': AddToken(Token.TokenType.LEFT_PAREN); break;
				case ')': AddToken(Token.TokenType.RIGHT_PAREN); break;
				case '{': AddToken(Token.TokenType.LEFT_BRACE); break;
				case '}': AddToken(Token.TokenType.RIGHT_BRACE); break;
				case ',': AddToken(Token.TokenType.COMMA); break;
				case '.': AddToken(Token.TokenType.DOT); break;
				case '-': AddToken(Token.TokenType.MINUS); break;
				case '+': AddToken(Token.TokenType.PLUS); break;
				case ';': AddToken(Token.TokenType.SEMICOLON); break;
				case '*': AddToken(Token.TokenType.STAR); break;
				case '!':
					AddToken(Match('=') ? Token.TokenType.BANG_EQUAL : Token.TokenType.BANG);
					break;
				case '=':
					AddToken(Match('=') ? Token.TokenType.EQUAL_EQUAL : Token.TokenType.EQUAL);
					break;
				case '<':
					AddToken(Match('=') ? Token.TokenType.LESS_EQUAL : Token.TokenType.LESS);
					break;
				case '>':
					AddToken(Match('=') ? Token.TokenType.GREATER_EQUAL : Token.TokenType.GREATER);
					break;
				case '/':
					if (Match('/'))
					{
						while (Peek() != '\n' && !IsAtEnd()) Advance();
					}
					else
					{
						AddToken(Token.TokenType.SLASH);
					}
					break;
				case ' ':
				case '\r':
				case '\t':
					// Ignore whitespace.
					break;
				case '\n':
					line++;
					lineStart = current;
					break;

				case '"':
					HandleString();
					break;
				default:
					if (IsDigit(c)) HandleNumber();
					else if (IsAlpha(c)) HandleIdentifier();
					else
					{
						Program.Error(line, "Unrecognized character");
					}
					break;
			}
		}

		void AddToken(Token.TokenType type)
		{
			AddToken(type, null);
		}

		void AddToken(Token.TokenType type, object? literal)
		{
			string text = source.Substring(start, current - start);
			tokens.Add(new(type, text, literal, line));
		}

		char Advance()
		{
			return source[current++];
		}

		char Peek()
		{
			if (IsAtEnd()) return '\0';
			return source[current];
		}

		char PeekNext()
		{
			if (current + 1 >= source.Length) return '\0';
			return source[current + 1];
		}

		bool Match(char c)
		{
			if (IsAtEnd()) return false;

			if (source[current] != c) return false;

			current++;
			return true;
		}

		bool IsAtEnd()
		{
			return source.Length <= current;
		}

		bool IsDigit(char c) => c >= '0' && c <= '9';

		bool IsAlpha(char c) => (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c == '_');

		bool IsAlphaNumeric(char c) => IsDigit(c) || IsAlpha(c);

		void HandleString()
		{
			while (Peek() != '"' && !IsAtEnd())
			{
				if (Peek() == '\n')
				{
					line++;
					lineStart = current;
				}
				Advance();
				}

			if (IsAtEnd())
			{
				Program.Error(line, current - lineStart, "Unterminated string");
				return;
			}

			Advance();

			string value = source.Substring(start + 1, current - start - 2);
			value = backslashReplacerRegex.Replace(
							tabReplacerRegex.Replace(
								newlineReplacerRegex.Replace(value, m => m.Groups[1].Value + "\n"), m => m.Groups[1].Value + "\t"), "\\");
			AddToken(Token.TokenType.STRING, value);
		}

		void HandleNumber()
		{
			while (IsDigit(Peek())) Advance();

			if (Peek() == '.' && IsDigit(PeekNext()))
			{
				Advance();

				while (IsDigit(Peek())) Advance();
			}

			AddToken(Token.TokenType.NUMBER, Double.Parse(source.Substring(start, current - start)));
		}

		void HandleIdentifier()
		{
			while (IsAlphaNumeric(Peek())) Advance();

			string text = source.Substring(start, current - start);

			if (Token.Keywords.ContainsKey(text))
			{
				AddToken(Token.Keywords[text]);
			}
			else
			{
				AddToken(Token.TokenType.IDENTIFIER);
			}
		}
	}
}
