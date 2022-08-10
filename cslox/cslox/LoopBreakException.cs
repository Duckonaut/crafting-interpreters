namespace cslox
{
    internal class LoopBreakException : Exception
	{
		internal enum LoopBreakType
		{
			Break,
			Continue
		}

		internal LoopBreakType type;

		internal LoopBreakException(LoopBreakType type) { this.type = type; }
	}
}
