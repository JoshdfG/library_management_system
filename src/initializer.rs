use {
    crate::logic::Book,
    crate::logic::BookType,
    crate::logic::Role,
    crate::logic::User,
    std::io::{self, Write},
};

pub fn ititialize() {
    let book1 = Book {
        title: String::from("Zarahs deal"),
        author: String::from("Robin Sharma"),
        isbn: String::from("1232334"),
        available: true,
    };

    let book2 = Book {
        title: String::from("The rust book"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        isbn: String::from("1234567"),
        available: true,
    };

    // accessing associated functions
    let bo = User::show_borrowed_book();
    println!("{:?}", bo);

    let mut user1 = User {
        name: String::from("Joel"),
        role: Role::Member,
        borrowed_books: Vec::<Book>::new(),
    };

    let mut user2 = User {
        name: String::from("Josh"),
        role: Role::Librarian,
        borrowed_books: Vec::<Book>::new(),
    };

    let mut books = vec![book1, book2];

    println!("Enter the title of the book you want to borrow:");
    io::stdout().flush().unwrap();
    let mut book_title = String::new();
    io::stdin()
        .read_line(&mut book_title)
        .expect("Failed to read line");

    let book_to_borrow = books
        .iter()
        .position(|book| book.title == book_title.trim());

    if let Some(index) = book_to_borrow {
        let book = &mut books[index];
        let book_type = match book.title.as_str() {
            "Zarahs deal" => BookType::Fiction,
            "The monk who sold his ferrari" => BookType::NonFiction,
            "The rust book" => BookType::Reference,
            _ => BookType::Magazine,
        };
        println!("Enter the name of the user who will borrow the book:");
        io::stdout().flush().unwrap();
        let mut borrower_name = String::new();
        io::stdin()
            .read_line(&mut borrower_name)
            .expect("Failed to read line");

        let borrower = match borrower_name.trim() {
            "Joel" => &mut user1,
            "Josh" => &mut user2,
            _ => panic!("Invalid borrower name"),
        };

        borrower.borrow_book(book, book_type);
        let borrowed_books = &borrower.show_borrowed_books();
        println!("{:?}", borrowed_books);
    } else {
        println!(
            "Sorry, we couldn't find a book titled '{}'. Please try again.",
            book_title.trim()
        );
    }

    println!("Enter the title of the book you want to return:");
    io::stdout().flush().unwrap();
    let mut book_title_return = String::new();
    io::stdin()
        .read_line(&mut book_title_return)
        .expect("Failed to read line");

    let book_to_return = books
        .iter()
        .position(|book| book.title == book_title_return.trim());

    if let Some(index) = book_to_return {
        let book = &mut books[index];
        let book_type = match book.title.as_str() {
            "Zarahs deal" => BookType::Fiction,
            "The monk who sold his ferrari" => BookType::NonFiction,
            "The rust book" => BookType::Reference,
            _ => BookType::Magazine,
        };
        println!("Enter the name of the user who will return the book:");
        io::stdout().flush().unwrap();
        let mut returners_name = String::new();
        io::stdin()
            .read_line(&mut returners_name)
            .expect("Failed to read line");

        let returner = match returners_name.trim() {
            "Joel" => &mut user1,
            "Josh" => &mut user2,
            _ => panic!("Invalid borrower name"),
        };

        returner.return_book(book, book_type);
    } else {
        println!(
            "Sorry, we couldn't find a book titled '{}'. Please try again.",
            book_title_return.trim()
        );
    }
}
