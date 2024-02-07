#[macro_export]
macro_rules! transform {
    () =>
        {
r"Transform and return the internal Rust data representation into the according Python
representation. If you do this mutiple times consider using the according `take_` method
to avoid an additional clone, however, be aware that the internal data is replaced with
its default value."
        }
}

#[macro_export]
macro_rules! take_transform {
    () =>
        {
r"Transform and return the internal Rust data representation into the according Python
representation replacing the internal data with its default value."
        }
}
