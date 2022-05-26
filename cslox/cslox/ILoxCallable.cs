using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace cslox
{
	internal interface ILoxCallable
	{
		int ArgumentCount { get; }
		object? Call(Interpreter interpreter, List<object?> args);
	}
}
