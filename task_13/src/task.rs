mod id_generator;

use std::cell::{Cell, RefCell};
use std::fmt::Display;
use crate::tree_print::*;

pub struct Task<P: TreePrinterTrait> {
    pub id: usize,
    pub title: Box<str>,
    pub completed: Cell<bool>,
    pub sub_tasks: RefCell<Vec<Box<Task<P>>>>,
    printer: RefCell<P>,
}

pub struct TaskFactory<P: TreePrinterCreator> {
    id_generator: id_generator::ID_Generator,
    tree_printer_factory: P,
}

impl<P: TreePrinterCreator> TaskFactory<P> {
    pub fn new(tree_printer_factory: P) -> Self {
        Self { id_generator: id_generator::ID_Generator::new(), tree_printer_factory }
    }

    pub fn create(&self, title: &str) -> Task<P::Output> {
        Task::new(title, &self.id_generator, self.tree_printer_factory.create())
    }
}

impl<P: TreePrinterTrait> Task<P> {
    pub fn new<G: id_generator::ID_Getter,>(title: &str, g: &G, printer: P) -> Self {
        let id = g.get_id();

        Task {
            id,
            title: title.to_string().into_boxed_str(),
            completed: Cell::new(false),
            sub_tasks: RefCell::new(Vec::new()),
            printer: RefCell::new(printer),
        }
    }

    pub fn mark_complete(&self) -> &Self {
        self.completed.set(true);
        self
    }

    pub fn mark_complete_recursive(&self) -> &Self {
        self.mark_complete();
        let tasks = self.sub_tasks.borrow();
        for task in tasks.iter().as_ref() {
            task.mark_complete_recursive();
        }
        self
    }

    pub fn add_subtask(&self, task: Task<P>) -> &Self {
        self.sub_tasks.borrow_mut().push(Box::new(task));
        self
    }

    pub fn print_tree(&self, indent: usize) {
        self.printer.borrow_mut().add_row(format!("{}", self), indent);
        println!("{}", self.printer.borrow_mut().result());

        for subtask in self.sub_tasks.borrow().iter() {
            subtask.print_tree(indent + 1);
        }
    }
}

impl<P: TreePrinterTrait> Display for Task<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let is_completed = if self.completed.get() {
            "✓"
        } else {
            "x"
        };
        write!(f, "[{}] {}", is_completed, self.title)
    }
}