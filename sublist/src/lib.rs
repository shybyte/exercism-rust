#[derive(Eq, PartialEq, Debug)]
pub enum Comparison {
    Equal,
    Superlist,
    Sublist,
    Unequal,
}

pub fn contains<T>(needle: &[T], haystack: &[T]) -> bool
    where T: PartialEq
{
    needle.is_empty() || haystack.windows(needle.len()).any(|w| w == needle)
}

pub fn sublist<T>(a: &[T], b: &[T]) -> Comparison
    where T: PartialEq
{
    if a == b {
        return Comparison::Equal;
    } else if contains(a, b) {
        return Comparison::Sublist;
    } else if contains(b, a) {
        return Comparison::Superlist;
    }

    Comparison::Unequal
}
