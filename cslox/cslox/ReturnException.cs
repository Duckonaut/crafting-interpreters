namespace cslox
{
    internal class ReturnException : Exception
	{
		internal object? value;

		internal ReturnException(object? value) { this.value = value; }
	}
}
