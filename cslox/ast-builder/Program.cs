public class Program
{
	public static void Main(string[] args)
	{
		if (args.Length != 1)
		{
			Console.Error.WriteLine("Usage: generate-ast <output_dir>");
			return;
		}
		string outputDir = args[0];
		DefineAst(outputDir, "Expr", new List<string>() {
			"Assign   : Token name, IExpr value",
			"Binary   : IExpr left, Token op, IExpr right",
			"Call	  : IExpr callee, Token paren, List<IExpr> arguments",
			"Get	  : IExpr obj, Token name",
			"Grouping : IExpr expression",
			"Literal  : object? value",
			"Logical  : IExpr left, Token op, IExpr right",
			"Self     : Token keyword",
			"Super    : Token keyword, Token method",
			"Set      : IExpr obj, Token name, IExpr value",
			"Unary    : Token op, IExpr right",
			"Variable : Token name"
		});

		DefineAst(outputDir, "Stmt", new List<string>() {
			"Block		: List<IStmt> statements",
			"Class		: Token name, Variable superclass, List<Function> methods",
			"Expression : IExpr expression",
			"Function	: Token name, List<Token> parameters, IStmt body",
			"IfStmt		: IExpr condition, IStmt then, IStmt elseDo",
			"Var        : Token name, IExpr initializer",
			"VarMut     : Token name, IExpr initializer",
			"WhileStmt	: IExpr condition, IStmt then",
			"ReturnStmt : Token keyword, IExpr? toReturn",
			"BreakStmt  : ",
			"ContinueStmt: ",
		}, new List<string>() { "cslox.Expr" });
	}

	private static void DefineAst(string outputDir, string baseName, List<string> types, List<string>? usings = null)
	{
		string path = $"{outputDir}/{baseName}.cs";

		StreamWriter sw = new StreamWriter(path);

		sw.WriteLine("using System.Collections.Generic;");
		sw.WriteLine("using System;");

		if (usings != null)
		{
			foreach (string u in usings)
			{
				sw.WriteLine($"using {u};");
			}
		}

		sw.WriteLine();
		sw.WriteLine($"namespace cslox.{baseName}");
		sw.WriteLine('{');
		sw.WriteLine($"\tinternal interface I{baseName}");
		sw.WriteLine("\t{");
		sw.WriteLine($"\t\tpublic R Accept<R>(I{baseName}Visitor<R> visitor);");
		sw.WriteLine("\t}");


		DefineVisitor(sw, baseName, types);

		foreach (string type in types)
		{
			string className = type.Split(':')[0].Trim();
			string fields = type.Split(':')[1].Trim();

			if (fields.Length == 0) fields = null;

			DefineType(sw, baseName, className, fields);
		}
		sw.WriteLine('}');

		sw.Flush();
		sw.Close();

	}

	private static void DefineType(StreamWriter sw, string baseName, string className, string? fieldList)
	{
		sw.WriteLine($"\tinternal class {className} : I{baseName}");
		sw.WriteLine("\t{");

		if (fieldList != null)
		{
			string[] fields = fieldList.Split(", ");

			foreach (string field in fields)
			{
				sw.WriteLine($"\t\tinternal {field};");
			}
		}

		sw.WriteLine($"\t\tinternal {className}({fieldList})");
		sw.WriteLine("\t\t{");

		if (fieldList != null)
		{
			string[] fields = fieldList.Split(", ");

			foreach (string field in fields)
			{
				string name = field.Split(' ')[1];
				sw.WriteLine($"\t\t\tthis.{name} = {name};");
			}
		}
		sw.WriteLine("\t\t}");

		sw.WriteLine($"\t\tpublic R Accept<R>(I{baseName}Visitor<R> visitor) => visitor.Visit{className}{baseName}(this);");
		sw.WriteLine("\t}");
	}

	private static void DefineVisitor(StreamWriter sw, string baseName, List<string> types)
	{
		sw.WriteLine($"\tinternal interface I{baseName}Visitor<R>");
		sw.WriteLine("\t{");

		foreach (string type in types)
		{
			string typeName = type.Split(':')[0].Trim();
			sw.WriteLine($"\t\tR Visit{typeName}{baseName}({typeName} {typeName.ToLower()});");
		}

		sw.WriteLine("\t}");
	}
}