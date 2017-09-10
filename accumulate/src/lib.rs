pub fn map_function<T, U>(input: Vec<T>, f: &Fn(T) -> U) -> Vec<U> where
{
    map_closure(input, f)
}

pub fn map_closure<T, U, F>(input: Vec<T>, f: F) -> Vec<U> where
    F: Fn(T) -> U,
{
    let mut result = Vec::with_capacity(input.len());
    for x in input {
        result.push(f(x));
    }
    result
}
