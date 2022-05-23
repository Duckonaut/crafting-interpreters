﻿using cslox.Expr;
using System.Diagnostics;
using System.Text;

namespace cslox
{
	public class Program
	{
		static Interpreter interpreter  = new Interpreter();
		static bool hadError = false;
		static bool hadRuntimeError = false;

		static string? currentSource;

		public static void Main(string[] args)
		{
			if (args.Length > 1)
			{
				Console.WriteLine("Usage: cslox [script]");
				return;
			}
			else if (args.Length == 1)
			{
				RunFile(args[0]);
			}
			else
			{
				RunPrompt();
			}
		}

		private static void RunFile(String path)
		{
			string s = File.ReadAllText(path);
			currentSource = s;
			Run(s);

			if (hadError) Environment.Exit(65);
			if (hadRuntimeError) Environment.Exit(70);
		}

		private static void RunPrompt()
		{
			while (true)
			{
				Console.Write("> ");
				string? line = Console.ReadLine();
				if (line == null) break;
				currentSource = line;
				Run(line);
				hadError = false;
			}
		}

		private static void Run(string source)
		{
			Lexer lexer = new(source);
			List<Token> tokens = lexer.GetTokens();

			Parser parser = new Parser(tokens);
			IExpr expression = parser.Parse();

			if (hadError) return;

			Console.WriteLine(new AstPrinter().Print(expression));

			interpreter.Interpret(expression);
		}

		public static void Error(int line, string message)
		{
			Report(line, currentSource.Split('\n')[line - 1], -1, message);
			hadError = true;
		}

		public static void Error(int line, int charInLine, string message)
		{
			Report(line, currentSource.Split('\n')[line - 1], charInLine, message);
			hadError = true;
		}

		internal static void RuntimeError(RuntimeError err)
		{
			Console.Error.WriteLine($"{err.Message} [line {err.token.line} at {err.token.lexeme}]");
			hadRuntimeError = true;
		}

		private static void Report(int line, string where, int charInLine, string message)
		{
			Console.ForegroundColor = ConsoleColor.Red;
			Console.Error.WriteLine($"{line} | {where}");
			if (charInLine >= 0)
			{
				int n = line.ToString().Length;
				for (int i = 0; i < n; i++)
					Console.Error.Write(' ');

				Console.Error.Write($" | ");
				for (int i = 0; i < charInLine; i++)
					Console.Error.Write(' ');
				Console.Error.WriteLine('^');
			}
			Console.ForegroundColor = ConsoleColor.White;

			Console.Error.WriteLine(message);
		}
	}

	class AstPrinter : Expr.IExprVisitor<String>
	{
		public string Print(IExpr expr)
		{
			return expr.Accept(this);
		}

		public string VisitBinaryExpr(Binary binary)
		{
			return WrapInParentheses(binary.op.lexeme, binary.left, binary.right);
		}

		public string VisitGroupingExpr(Grouping grouping)
		{
			return WrapInParentheses("group", grouping.expression);
		}

		public string VisitLiteralExpr(Literal literal)
		{
			if (literal.value is string s) return $"\"{s}\"";
			return literal?.value.ToString();
		}

		public string VisitUnaryExpr(Unary unary)
		{
			return WrapInParentheses(unary.op.lexeme, unary.right);
		}

		private string WrapInParentheses(string name, params IExpr[] args)
		{
			StringBuilder sb = new();
			sb.Append("(");
			sb.Append(name);

			foreach (var arg in args)
			{
				sb.Append(' ');
				sb.Append(arg.Accept(this));
			}

			sb.Append(")");
			return sb.ToString();
		}
	}
}	