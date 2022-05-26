﻿using cslox.Stmt;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace cslox
{
	internal class LoxFunction : ILoxCallable
	{
		Function declaration;

		public LoxFunction(Function declaration)
		{
			this.declaration = declaration;
		}

		public int ArgumentCount => declaration.parameters.Count;
		public object? Call(Interpreter interpreter, List<object?> args)
		{
			Environment environment = new Environment(interpreter.globals);

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
				return r.value;
			}
			return null;
		}

		public override string ToString()
		{
			return $"<fn {declaration.name.lexeme}>";
		}
	}
}
