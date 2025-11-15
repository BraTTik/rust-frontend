type ID = u32;
type IsAvailable = bool;

struct Author <'a>{
    name: &'a str,
    birth_year: u16,
}

struct Book <'a>{
    year: u16,
    name: &'a str,
    author: Author<'a>,
    tags: Vec<&'a str>,
    copies: Vec<(ID, IsAvailable)>,
}

struct Library<'a > {
    books: Vec<Book<'a>>,
}

fn count_available_copies(book: &Book) -> usize {
    let mut count = 0;
    for copy in &book.copies {
        if copy.1 == true {
            count += 1
        }
    }

    count
}

fn find_books_by_author<'a>(library: &'a Library, name: &'a str) -> Box<[&'a Book<'a>]> {
    let mut result: Vec<&Book> = Vec::new();

    for book in &library.books {
        if book.author.name == name {
            result.push(book);
        }
    }

    result.into_boxed_slice()
}

fn add_tag<'a, 'b>(book: &'a mut Book<'b>, tag: &'b str) {
    book.tags.push(tag);
}

fn oldest_book<'a>(library: &'a Library) -> Option<&'a Book<'a>> {
    let mut result: Option<&'a Book> = None;

    for book in &library.books {
        if result.is_none() || book.year < result?.year {
            result = Some(book)
        }
    }

    result
}

fn main() {
    let mut book1 = Book {
        name: "Солярис",
        year: 1961,
        author: Author { name: "Лем", birth_year: 1921 },
        tags: vec!["sci-fi"],
        copies: vec![(1, true), (2, false), (3, true)],
    };

    add_tag(&mut book1, "classic");
    add_tag(&mut book1, "sci-fi");

    let book2 = Book {
        name: "Пикник на обочине",
        year: 1972,
        author: Author { name: "Стругацкие", birth_year: 1933 },
        tags: vec!["classic", "sci-fi"],
        copies: vec![(10, false), (11, false)],
    };

    let library = Library {
        books: vec![book1, book2],
    };

    let books_lem = find_books_by_author(&library, "Лем");
    assert_eq!(books_lem.len(), 1);
    assert_eq!(books_lem[0].name, "Солярис");

    let books_str = find_books_by_author(&library, "Стругацкие");
    assert_eq!(books_str.len(), 1);
    assert_eq!(books_str[0].name, "Пикник на обочине");

    let oldest = oldest_book(&library).unwrap();
    assert_eq!(oldest.name, "Солярис");

    let first_book = &library.books[0];
    assert_eq!(count_available_copies(&first_book), 2);

    println!("Все тесты прошли!");
}
