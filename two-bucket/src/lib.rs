use std::cmp::{max, min};

#[derive(Eq, PartialEq, Clone, Copy)]
struct State(u8, u8);

impl State {
    fn pour_1_into_2(&self, capacity_2: u8) -> State {
        State(
            max(self.0 as i32 - (capacity_2 as i32 - self.1 as i32), 0) as u8,
            min(self.1 + self.0, capacity_2)
        )
    }

    fn pour_2_into_1(&self, capacity_1: u8) -> State {
        State(
            min(self.0 + self.1, capacity_1),
            max(self.1 as i32 - (capacity_1 as i32 - self.0 as i32), 0) as u8
        )
    }

    // Returns also forbidden states, that needs to get filtered later
    fn next_states(&self,
                   capacity_1: u8,
                   capacity_2: u8, ) -> Vec<State> {
        vec![
            State(capacity_1, self.1), // filling bucket one
            State(self.0, capacity_2), // filling bucket two
            State(0, self.1), // empty bucket one
            State(self.0, 0), // empty bucket two
            self.pour_1_into_2(capacity_2),
            self.pour_2_into_1(capacity_1),
        ]
    }
}


#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Bucket {
    One,
    Two
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}


/// Solve the bucket problem
pub fn solve(capacity_1: u8,
             capacity_2: u8,
             goal: u8,
             start_bucket_ref: &Bucket) -> BucketStats
{
    let start_bucket = *start_bucket_ref;

    let initial_state = match start_bucket {
        Bucket::One => State(capacity_1, 0),
        Bucket::Two => State(0, capacity_2)
    };

    let forbidden_state = match start_bucket {
        Bucket::One => State(0, capacity_2),
        Bucket::Two => State(capacity_1, 0)
    };

    let mut states_stack = vec![initial_state];
    return search_moves(&mut states_stack, capacity_1, capacity_2, goal, forbidden_state).unwrap();
}

fn search_moves(states_stack: &mut Vec<State>, capacity_1: u8,
                capacity_2: u8, goal: u8, forbidden_state: State) -> Option<BucketStats> {
    let state = states_stack.last().unwrap().clone();

    if state.0 == goal {
        return Some(BucketStats {
            moves: states_stack.len() as u8,
            goal_bucket: Bucket::One,
            other_bucket: state.1
        });
    } else if state.1 == goal {
        return Some(BucketStats {
            moves: states_stack.len() as u8,
            goal_bucket: Bucket::Two,
            other_bucket: state.0
        });
    }

    let new_legal_states: Vec<State> = state.next_states(capacity_1, capacity_2).into_iter()
        .filter(|&s| s != forbidden_state && !states_stack.contains(&s)).collect();

    for new_state in new_legal_states {
        states_stack.push(new_state);
        let result = search_moves(states_stack, capacity_1, capacity_2, goal, forbidden_state);
        states_stack.pop();
        if result.is_some() {
            return result;
        }
    }

    return None;
}