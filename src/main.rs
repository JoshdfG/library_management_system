use std::io::{self, Write};

#[derive(Clone)]
struct Book {
    title: String,
    author: String,
    isbn: String,
    available: bool,
}

enum Role {
    Member(String),
    Librarian(String),
}

#[derive(PartialEq)]
enum BookType {
    Fiction,
    NonFiction,
    Reference,
    Magazine,
}

struct User {
    name: String,
    role: Role,
    borrowed_books: Vec<Book>,
}

impl Book {
    fn display(&self) {
        println!(
            "Title :{} Author:{} ISBN:{} Availability:{}",
            self.title, self.author, self.isbn, self.available
        );
    }
}

impl User {
    fn borrow_book(&mut self, book: &mut Book, book_type: BookType) {
        match self.role {
            Role::Member(_) => {
                if book_type == BookType::Reference {
                    println!("Members cant borrow reference types book")
                } else if !book.available {
                    println!("The requested book is not available");
                } else {
                    book.available = false;
                    self.borrowed_books.push(book.clone());
                    println!("{} borrowed {}", self.name, book.title);
                }
            }
            Role::Librarian(_) => {
                if book.available {
                    book.available = false;
                    self.borrowed_books.push(book.clone());
                    println!("{} borrowed {}", self.name, book.title);
                } else {
                    println!("The requested book is not available");
                }
            }
        }
    }

    fn return_book(&mut self, book: &mut Book, book_type: BookType) {
        match self.role {
            Role::Member(_) => {
                book.available = true;
                let book_index = self
                    .borrowed_books
                    .iter()
                    .position(|b| b.title == book.title);
                if let Some(index) = book_index {
                    self.borrowed_books.remove(index);
                    println!("{} returned {}", self.name, book.title);
                }
                println!("not found");
            }
            Role::Librarian(_) => {
                book.available = true;
                let book_index = self
                    .borrowed_books
                    .iter()
                    .position(|b| b.title == book.title);
                if let Some(index) = book_index {
                    self.borrowed_books.remove(index);
                    println!("{} returned {}", self.name, book.title);
                } else {
                    println!("not founds")
                }
            }
        }
    }
}
fn main() {
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

    let mut user1 = User {
        name: String::from("Joel"),
        role: Role::Member("Joel".to_owned()),
        borrowed_books: Vec::<Book>::new(),
    };

    let mut user2 = User {
        name: String::from("Josh"),
        role: Role::Librarian("Josh".to_owned()),
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
