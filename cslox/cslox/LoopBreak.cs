using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace cslox
{
	internal class LoopBreak
	{
		internal enum LoopBreakType
		{
			Break,
			Continue
		}

		internal LoopBreakType type;

		internal LoopBreak(LoopBreakType type) { this.type = type; }
	}
}
