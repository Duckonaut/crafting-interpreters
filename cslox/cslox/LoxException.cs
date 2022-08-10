namespace cslox
{
    internal class LoxException : Exception
	{
		string why;
		int line;
		public LoxException(string why, int line) : base($"{why} at {line}")
		{
			this.line = line;
			this.why = why;
		}
	}
}
