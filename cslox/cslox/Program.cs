using System.Diagnostics;

namespace cslox
{
	public class Program
	{
		static bool hadError = false;

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
}	