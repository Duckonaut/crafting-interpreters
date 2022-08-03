using cslox.Expr;
using cslox.Stmt;
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

			if (hadError) System.Environment.Exit(65);
			if (hadRuntimeError) System.Environment.Exit(70);
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
			List<IStmt> expression = parser.Parse();

			if (hadError) return;
			foreach (IStmt stmt in expression)
				Console.WriteLine(new AstPrinter().Print(stmt));

			Resolver resolver = new Resolver(interpreter);
			resolver.Resolve(expression);

			if (hadError) return;

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

	class AstPrinter : Expr.IExprVisitor<String>, IStmtVisitor<String>
	{
		public string Print(IExpr expr)
		{
			return expr.Accept(this);
		}

		public string Print(IStmt stmt)
		{
			return stmt.Accept(this);
		}

		public string VisitAssignExpr(Assign assign)
		{
			return WrapInParentheses($"(assign {assign.name.lexeme}", assign.value);
		}

		public string VisitBinaryExpr(Binary binary)
		{
			return WrapInParentheses(binary.op.lexeme, binary.left, binary.right);
		}

		public string VisitBlockStmt(Block block)
		{
			return WrapInParentheses("block", block.statements.ToArray());
		}

		public string VisitBreakStmtStmt(BreakStmt breakstmt)
		{
			return "break";
		}

		public string VisitCallExpr(Call call)
		{
			return $"(call callee: {call.callee.Accept(this)} {WrapInParentheses("arguments", call.arguments.ToArray())})";
		}

		public string VisitContinueStmtStmt(ContinueStmt continuestmt)
		{
			return "continue";
		}

		public string VisitExpressionStmt(Expression expression)
		{
			return expression.expression.Accept(this);
		}

		public string VisitFunctionStmt(Function function)
		{
			var s = $"(function name: {function.name.lexeme}";
			if (function.parameters.Count > 0)
				s += $"(parameters {function.parameters.Select(t => t.lexeme).Aggregate((s, p) => s += " " + p)})";
			s += $" {function.body.Accept(this)})";
			return s;
		}

		public string VisitGroupingExpr(Grouping grouping)
		{
			return WrapInParentheses("group", grouping.expression);
		}

		public string VisitIfStmtStmt(IfStmt ifstmt)
		{
			return $"(if ({ifstmt.condition.Accept(this)}) (then {ifstmt.then.Accept(this)}) (else {ifstmt.elseDo?.Accept(this)}))";
		}

		public string VisitLiteralExpr(Literal literal)
		{
			if (literal.value is string s) return $"\"{s}\"";
			if (literal.value == null) return "nil";
			return literal?.value.ToString();
		}

		public string VisitLogicalExpr(Logical logical)
		{
			return WrapInParentheses(logical.op.lexeme, logical.left, logical.right);
		}

		public string VisitReturnStmtStmt(ReturnStmt returnstmt)
		{
			return WrapInParentheses("return", returnstmt.toReturn);
		}

		public string VisitUnaryExpr(Unary unary)
		{
			return WrapInParentheses(unary.op.lexeme, unary.right);
		}

		public string VisitVariableExpr(Variable variable)
		{
			return $"(variable {variable.name.lexeme})";
		}

		public string VisitVarMutStmt(VarMut varmut)
		{
			return WrapInParentheses($"(var mut {varmut.name.lexeme})", varmut.initializer);
		}

		public string VisitVarStmt(Var var)
		{
			return WrapInParentheses($"(var {var.name.lexeme})", var.initializer);
		}

		public string VisitWhileStmtStmt(WhileStmt whilestmt)
		{
			return WrapInParentheses($"while ({whilestmt.condition.Accept(this)})", whilestmt.then);
		}

		private string WrapInParentheses(string name, params IExpr[] args)
		{
			StringBuilder sb = new();
			sb.Append("(");
			sb.Append(name);

			foreach (var arg in args)
			{
				sb.Append(' ');
				if (arg == null)
					sb.Append("nil");
				else
					sb.Append(arg.Accept(this));
			}

			sb.Append(")");
			return sb.ToString();
		}

		private string WrapInParentheses(string name, params IStmt[] args)
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