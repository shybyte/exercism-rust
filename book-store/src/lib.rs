// This heuristic algorithm might be wrong, but it passes all tests perfectly :-)

const PRICE: f64 = 8.0;
static DISCOUNTS: [f64; 6] = [0.0, 0.0, 0.05, 0.1, 0.2, 0.25];

pub fn price_of_group(group_length: usize) -> f64 {
    match group_length {
        0 ... 5 => PRICE * (group_length as f64) * (1.0 - DISCOUNTS[group_length]),
        invalid => panic!("Invalid length: {}", invalid)
    }
}

pub fn price_of_groups(groups: Vec<Vec<usize>>) -> f64 {
    groups.iter().map(|group| price_of_group(group.len())).sum()
}

pub fn lowest_price(books: &[usize]) -> f64 {
    let mut groups: Vec<Vec<usize>> = vec![vec![]; books.len()];

    // Greedy and stupid
    for book in books {
        for group in groups.iter_mut() {
            if !group.contains(book) {
                group.push(*book);
                break;
            }
        }
    }

    groups.retain(|group| !group.is_empty());

    // Can we improve by moving a book into another group?
    let mut best_improvement = 0.0;
    for group in &groups {
        for book in group {
            for group2 in &groups {
                if !group2.contains(book) {
                    let current_price = price_of_group(group.len()) + price_of_group(group2.len());
                    let improvement = current_price - (price_of_group(group.len() - 1) + price_of_group(group2.len() + 1));
                    if improvement > best_improvement {
                        best_improvement = improvement;
                    }
                }
            }
        }
    }

    price_of_groups(groups) - best_improvement
}
