use std::cell::Cell;

pub trait ID_Getter {
    fn get_id(&self) -> usize;
}

pub struct ID_Generator {
    next_id: Cell<usize>
}

impl ID_Generator {
    pub fn new() -> Self {
        Self::start_from(1)
    }

    pub fn start_from(start_with: usize) -> Self {
        ID_Generator { next_id: Cell::new(start_with) }
    }

}

impl ID_Getter for ID_Generator {
    fn get_id(&self) -> usize {
        let id = self.next_id.get();
        self.next_id.set(id + 1);
        id
    }
}