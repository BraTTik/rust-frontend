use std::iter::Peekable;

struct RLE<I: Iterator> {
    i: Peekable<I>,
}

impl <I: Iterator> RLE<I> {
    fn new(i: I) -> Self {
        Self { i: i.peekable() }
    }
}

impl <I> Iterator for RLE<I>
where I: Iterator,
    I::Item: Eq + Copy
{
    type Item = (I::Item, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.i.next();

        match current {
            None => None,
            Some(current) => {
                let mut count = 1usize;
                while let Some(next) = self.i.peek() {

                    if current == *next {
                        count += 1;
                        self.i.next();
                    } else {
                        break;
                    }

                }

                Some((current, count))
            }
        }
    }
}

pub fn run_length_encode(s: &str) -> impl Iterator<Item = (char, usize)> {
    RLE::new(s.chars())
}