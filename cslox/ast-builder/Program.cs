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
			"Grouping : IExpr expression",
			"Literal  : object? value",
			"Unary    : Token op, IExpr right",
			"Variable : Token name"
		});

		DefineAst(outputDir, "Stmt", new List<string>() {
			"Block		: List<IStmt> statements",
			"Expression : IExpr expression",
			"Print      : IExpr expression",
			"Var        : Token name, IExpr initializer",
			"VarMut     : Token name, IExpr initializer"
		});
	}

	private static void DefineAst(string outputDir, string baseName, List<string> types)
	{
		string path = $"{outputDir}/{baseName}.cs";

		StreamWriter sw = new StreamWriter(path);

		sw.WriteLine("using System.Collections.Generic;");
		sw.WriteLine("using System;");
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

			DefineType(sw, baseName, className, fields);
		}
		sw.WriteLine('}');

		sw.Flush();
		sw.Close();

	}

	private static void DefineType(StreamWriter sw, string baseName, string className, string fieldList)
	{
		sw.WriteLine($"\tinternal class {className} : I{baseName}");
		sw.WriteLine("\t{");
		string[] fields = fieldList.Split(", ");

		foreach (string field in fields)
		{
			sw.WriteLine($"\t\tinternal {field};");
		}

		sw.WriteLine($"\t\tinternal {className}({fieldList})");
		sw.WriteLine("\t\t{");
		foreach (string field in fields)
		{
			string name = field.Split(' ')[1];
			sw.WriteLine($"\t\t\tthis.{name} = {name};");
		}
		sw.WriteLine("\t\t}");

		sw.WriteLine($"\t\tpublic R Accept<R>(I{baseName}Visitor<R> visitor) => visitor.Visit{className}{baseName}(this);");
		sw.WriteLine("\t}");
	}

	private static void DefineVisitor(StreamWriter sw, string baseName, List<string> types)
	{
		sw.WriteLine($"\tinternal interface I{baseName}Visitor<R>");
		sw.WriteLine("\t{");
		
		foreach(string type in types)
		{
			string typeName = type.Split(':')[0].Trim();
			sw.WriteLine($"\t\tR Visit{typeName}{baseName}({typeName} {typeName.ToLower()});");
		}

		sw.WriteLine("\t}");
	}
}