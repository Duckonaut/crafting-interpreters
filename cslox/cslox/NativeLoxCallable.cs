namespace cslox
{
    internal class NativeLoxCallable : ILoxCallable
	{
		private int argumentCount;
		private Func<Interpreter, List<object?>, object?> func;

		public NativeLoxCallable(int argumentCount, Func<Interpreter, List<object?>, object?> func)
		{
			this.argumentCount = argumentCount;
			this.func = func;
		}

		public int ArgumentCount => argumentCount;
		public object? Call(Interpreter interpreter, List<object?> args) => func.Invoke(interpreter, args);

		public override string ToString()
		{
			return "<native fn>";
		}
	}
}
