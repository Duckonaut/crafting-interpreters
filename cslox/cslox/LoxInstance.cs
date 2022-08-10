namespace cslox
{
    internal class LoxInstance
    {
        private LoxClass cl;
        private Dictionary<string, object?> fields = new Dictionary<string, object?>();

        public LoxInstance(LoxClass cl)
        {
            this.cl = cl;
        }

        public override string ToString()
        {
            return "<" + cl.name + " instance>";
        }

        internal object? Get(Token name)
        {
            if (fields.ContainsKey(name.lexeme))
            {
                return fields[name.lexeme];
            }

            LoxFunction? method = cl.FindMethod(name.lexeme);
            if (method != null) return method.Bind(this);

            throw new RuntimeError(name, "Undefined propery '" + name.lexeme + "'.");
        }

        internal void Set(Token name, object? value)
        {
            fields[name.lexeme] = value;
        }
    }
}
