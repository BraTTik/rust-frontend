pub trait TreePrinterTrait: Clone {
    fn add_row(&mut self, row: String, indent: usize);
    fn result(&mut self) -> String;
}

pub trait TreePrinterCreator {
    type Output: TreePrinterTrait;

    fn create(&self) -> Self::Output;
}


pub struct TreePrinterFactory {
    pub indent_size: usize,
}

impl TreePrinterFactory {
    pub fn new(indent_size: usize) -> Self {
        Self { indent_size }
    }
}

impl TreePrinterCreator for TreePrinterFactory {
    type Output = TreePrinter;

    fn create(&self) -> TreePrinter {
        TreePrinter::new(self.indent_size)
    }
}

#[derive(Debug, Clone,)]
pub struct TreePrinter {
    indent_size: usize,
    result: String,
}

impl TreePrinter {
    pub fn new(indent_size: usize) -> Self {
        Self { result: String::new(), indent_size }
    }
}

impl TreePrinterTrait for TreePrinter {
    fn add_row(&mut self, row: String, indent: usize) {
        self.result.push_str(&format!("{:indent$}{row}\n", ' ', indent = indent * self.indent_size, row = row));
    }

    fn result(&mut self) -> String {
        let r = self.result.clone();
        self.result.clear();
        r
    }
}
