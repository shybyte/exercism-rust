// This heuristic algorithm scales badly and might be wrong,
// but it passes all tests perfectly :-)
// lowest_price(&vec![1; 10_000]) in release mode needs less than 1 second on my old computer
// but with more books is gets slow pretty soon -> O(n^3)

const PRICE: f64 = 8.0;
static DISCOUNTS: [f64; 6] = [0.0, 0.0, 0.05, 0.1, 0.2, 0.25];

fn price_of_group(group_length: usize) -> f64 {
    match group_length {
        0 ... 5 => PRICE * (group_length as f64) * (1.0 - DISCOUNTS[group_length]),
        invalid => panic!("Invalid length: {}", invalid)
    }
}

fn price_of_groups(groups: Vec<Vec<usize>>) -> f64 {
    groups.iter().map(|group| price_of_group(group.len())).sum()
}

type BookIndex = usize;
type SourceGroupIndex = usize;
type TargetGroupIndex = usize;

/// Can we improve by moving one book into another group?
fn find_best_improvement(groups: &Vec<Vec<usize>>) -> Option<(BookIndex, SourceGroupIndex, TargetGroupIndex)> {
    let mut best_improvement_price_delta = 0.0;
    let mut best_improvement_move = None;

    for (src_i, src_group) in groups.iter().enumerate() {
        for (book_i, book) in src_group.iter().enumerate() {
            for (target_i, target_group) in groups.iter().enumerate() {
                if !target_group.contains(book) {
                    let current_price = price_of_group(src_group.len()) + price_of_group(target_group.len());
                    let improvement_price_delta = current_price - (price_of_group(src_group.len() - 1) + price_of_group(target_group.len() + 1));
                    if improvement_price_delta > best_improvement_price_delta {
                        best_improvement_price_delta = improvement_price_delta;
                        best_improvement_move = Some((book_i, src_i, target_i));
                    }
                }
            }
        }
    }

    best_improvement_move
}


pub fn lowest_price(books: &[usize]) -> f64 {
    let mut groups: Vec<Vec<usize>> = vec![vec![]; books.len()];

    // Fill the groups in the most stupid boring way
    for book in books {
        for group in groups.iter_mut() {
            if !group.contains(book) {
                group.push(*book);
                break;
            }
        }
    }

    // Remove empty groups
    groups.retain(|group| !group.is_empty());

    // Improve it greedily until there is no improvement anymore
    loop {
        if let Some((book_index, source_group, target_group)) = find_best_improvement(&groups) {
            let book = groups[source_group].remove(book_index);
            groups[target_group].push(book);
        } else {
            break;
        }
    }

    // Round it, because the tests are pretty sensible to minimal floating point errors
    (price_of_groups(groups) * 100.0).round() / 100.0
}
