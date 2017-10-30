use std::cmp::Ordering;

pub fn find<T: Ord, C: AsRef<[T]>>(elements: C, needle: T) -> Option<usize>
{
    let slice: &[T] = elements.as_ref();
    let (head, middle_and_tail) = slice.split_at(slice.len() / 2);
    middle_and_tail.first().and_then(|middle_element|
        match needle.cmp(middle_element) {
            Ordering::Less => find(head, needle),
            Ordering::Equal => Some(head.len()),
            Ordering::Greater => find(&middle_and_tail[1..], needle)
                .map(|i| i + head.len() + 1)
        }
    )
}