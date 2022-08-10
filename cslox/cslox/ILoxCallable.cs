namespace cslox
{
    internal interface ILoxCallable
	{
		int ArgumentCount { get; }
		object? Call(Interpreter interpreter, List<object?> args);
	}
}
