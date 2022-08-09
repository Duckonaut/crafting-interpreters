using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace cslox
{
	internal enum ClassType
	{
		None,
		Class,
		Subclass
	}
	internal class LoxClass : ILoxCallable
	{
		public string name;
		private Dictionary<string, LoxFunction> methods;
		private LoxClass superclass;

		public LoxClass(string name, LoxClass superclass, Dictionary<string, LoxFunction> methods)
		{
			this.name = name;
			this.superclass = superclass;
			this.methods = methods;
		}

		public int ArgumentCount
		{
			get
			{
				LoxFunction? initializer = FindMethod("init");
				if (initializer == null) return 0;

				return initializer.ArgumentCount;
			}
		}

		public object? Call(Interpreter interpreter, List<object?> args)
		{
			LoxInstance instance = new LoxInstance(this);

			LoxFunction initializer = FindMethod("init");

			if (initializer != null)
			{
				initializer.Bind(instance).Call(interpreter, args);
			}

			return instance;
		}

		public override string ToString()
		{
			return "<class " + name + ">";
		}

		internal LoxFunction? FindMethod(string name)
		{
			if (methods.ContainsKey(name))
			{
				return methods[name];
			}

			if (superclass != null)
			{
				return superclass.FindMethod(name);
			}

			return null;
		}
	}
}
