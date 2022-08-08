using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace cslox
{
	internal class LoxClass : ILoxCallable
	{
		public string name;

		public LoxClass(string name)
		{
			this.name = name;
		}

		public int ArgumentCount => 0;

		public object? Call(Interpreter interpreter, List<object?> args)
		{
			LoxInstance instance = new LoxInstance(this);
			return instance;
		}

		public override string ToString()
		{
			return "<class " + name + ">";
		}
	}
}
