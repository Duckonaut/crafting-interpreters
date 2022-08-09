using cslox.Stmt;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace cslox
{
	internal enum FunctionType
	{
		None,
		Function,
		Method,
		Initializer
	}
	internal class LoxFunction : ILoxCallable
	{
		Function declaration;
		private Environment closure;
		private bool isInitializer;

		public LoxFunction(Function declaration, Environment closure, bool isInitializer)
		{
			this.declaration = declaration;
			this.closure = closure;
			this.isInitializer = isInitializer;
		}

		public int ArgumentCount => declaration.parameters.Count;
		public object? Call(Interpreter interpreter, List<object?> args)
		{
			Environment environment = new Environment(closure);

			for (int i = 0; i < declaration.parameters.Count; i++)
			{
				environment.Define(declaration.parameters[i], args[i]);
			}
			try
			{
				interpreter.ExecuteBlock((declaration.body as Block).statements, environment);
			}
			catch (ReturnException r)
			{
				if (isInitializer) return closure.GetAt(0, "self");
				return r.value;
			}
			if (isInitializer) return closure.GetAt(0, "self");
			return null;
		}

		internal LoxFunction Bind(LoxInstance loxInstance)
		{
			var environment = new Environment(closure);
			environment.Define(new Token(Token.TokenType.BUILTIN, "self", null, -1), loxInstance);
			return new LoxFunction(declaration, environment, isInitializer);
		}

		public override string ToString()
		{
			return $"<fn {declaration.name.lexeme}>";
		}
	}
}
