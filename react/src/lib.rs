use std::collections::HashMap;

// Because these are passed without & to some functions,
// it will probably be necessary for these two types to be Copy.
pub type CellID = usize;
pub type CallbackID = usize;

type ComputationFunction<T> = Fn(&[T]) -> T;

struct Computation<T> {
    dependencies: Vec<CellID>,
    function: Box<ComputationFunction<T>>
}

struct Cell<'a, T> {
    value: T,
    last_value: T,
    dependent: Vec<CellID>,
    kind: CellKind<T>,
    callbacks: HashMap<CallbackID, Box<FnMut(T) -> () + 'a>>,
}

enum CellKind<T> {
    Input,
    Compute(Computation<T>)
}

pub struct Reactor<'a, T> {
    cells: Vec<Cell<'a, T>>,
    callback_id_counter: CallbackID
}

impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Reactor { cells: Vec::new(), callback_id_counter: 0 }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> CellID {
        self.cells.push(Cell {
            value: initial,
            last_value: initial,
            dependent: Vec::new(),
            kind: CellKind::Input,
            callbacks: HashMap::new()
        });
        self.cells.len() - 1
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // Return an Err (and you can change the error type) if any dependency doesn't exist.
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T + 'static>(&mut self, dependencies: &[CellID], compute_func: F) -> Result<CellID, ()> {
        if !dependencies.iter().all(|&id| id < self.cells.len()) {
            return Err(());
        }
        let dependencies_values: Vec<_> = dependencies.iter().map(|&id| self.value(id).unwrap()).collect();
        let value = compute_func(&dependencies_values);
        self.cells.push(
            Cell {
                value: value,
                last_value: value,
                dependent: Vec::new(),
                kind: CellKind::Compute(Computation {
                    dependencies: dependencies.into(),
                    function: Box::new(compute_func)
                }),
                callbacks: HashMap::new()
            }
        );

        let id = self.cells.len() - 1;

        for &dep_id in dependencies {
            self.cells.get_mut(dep_id).unwrap().dependent.push(id);
        }

        Ok(id)
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellID) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellID) -> Option<T> {
        self.cells.get(id).map(|&ref cell| cell.value)
    }

    // Sets the value of the specified input cell.
    //
    // Return an Err (and you can change the error type) if the cell does not exist, or the
    // specified cell is a compute cell, since compute cells cannot have their values directly set.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellID) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: CellID, new_value: T) -> Result<(), ()> {
        if self.cells.len() <= id {
            return Err(());
        }
        if let CellKind::Compute(_) = self.cells[id].kind {
            return Err(());
        }
        self.set_and_propagate_value(id, new_value);

        let dependent = self.cells[id].dependent.clone();
        for dep_id in dependent {
            self.fire_callbacks_id_needed(dep_id);
        }

        Ok(())
    }

    pub fn compute_value(&mut self, id: CellID) -> () {
        let new_value = if let CellKind::Compute(ref computation) = self.cells[id].kind {
            let dependencies_values: Vec<_> = computation.dependencies.iter().map(|&id| self.cells[id].value).collect();
            let ref f = computation.function;
            f(&dependencies_values)
        } else {
            panic!("compute_value of input makes no sense");
        };
        self.set_and_propagate_value(id, new_value);
    }

    fn set_and_propagate_value(&mut self, id: CellID, new_value: T) -> () {
        self.cells[id].value = new_value;
        let dependent = self.cells[id].dependent.clone();
        for dep_id in dependent {
            self.compute_value(dep_id);
        }
    }

    fn fire_callbacks_id_needed(&mut self, id: CellID) {
        let dependent = match self.cells.get_mut(id) {
            Some(c) => {
                if c.value == c.last_value {
                    return
                }
                for cb in c.callbacks.values_mut() {
                    cb(c.value);
                }
                c.last_value = c.value;
                c.dependent.clone()
            },
            None => panic!("Missing cell"),
        };

        for dep_id in dependent {
            self.fire_callbacks_id_needed(dep_id);
        }
    }

    // Adds a callback to the specified compute cell.
    //
    // Return an Err (and you can change the error type) if the cell does not exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) -> () + 'a>(&mut self, id: CellID, callback: F) -> Result<CallbackID, ()> {
        match self.cells.get_mut(id) {
            Some(cell) => {
                self.callback_id_counter += 1;
                cell.callbacks.insert(self.callback_id_counter, Box::new(callback));
                Ok(self.callback_id_counter)
            },
            None => Err(()),
        }
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Return an Err (and you can change the error type) if either the cell or callback
    // does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(&mut self, cell: CellID, callback: CallbackID) -> Result<(), ()> {
        match self.cells.get_mut(cell) {
            Some(c) => match c.callbacks.remove(&callback) {
                Some(_) => Ok(()),
                None => Err(()),
            },
            None => Err(()),
        }
    }
}
