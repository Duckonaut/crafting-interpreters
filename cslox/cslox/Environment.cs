using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace cslox
{
	internal class Environment
	{
		private Dictionary<string, VariableInstance> values = new Dictionary<string, VariableInstance>();
		private Environment enclosing;

		internal Environment() { enclosing = null; }

		internal Environment(Environment environment) { enclosing = environment; }

		public void Define(Token name, object? obj, bool mutable = false)
		{
			values[name.lexeme] = new VariableInstance(obj, mutable, name);
		}

		public void Assign(Token name, object? newValue)
		{
			if (values.ContainsKey(name.lexeme))
			{
				if (values[name.lexeme].mutable)
				{
					values[name.lexeme].value = newValue;
					return;
				}
				throw new RuntimeError(values[name.lexeme].definitionToken, "Cannot mutate a variable not marked as 'mut'");
			}

			if (enclosing != null)
			{
				enclosing.Assign(name, newValue);
				return;
			}

			throw new RuntimeError(name, $"Variable {name.lexeme} not defined.");
		}

		public object? Get(Token name)
		{
			if (values.ContainsKey(name.lexeme))
			{
				return values[name.lexeme].value;
			}
			
			if (enclosing != null) return enclosing.Get(name);

			throw new RuntimeError(name, $"Undefined variable: {name.lexeme}");
		}

		public bool Defined(Token name)
		{
			return values.ContainsKey(name.lexeme);
		}

		public bool Mutable(Token name)
		{
			return (Defined(name) && values[name.lexeme].mutable) || (enclosing != null && enclosing.Mutable(name));
		}
	}

	internal class VariableInstance
	{
		internal object? value;
		internal bool mutable;
		internal Token definitionToken;

		public VariableInstance(object? value, bool mutable, Token definitionToken)
		{
			this.value = value;
			this.mutable = mutable;
			this.definitionToken = definitionToken;
		}
	}
}
