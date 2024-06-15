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
                self.borrowed_books.pop();
                println!("{} returned {}", self.name, book.title);
            }
            Role::Librarian(_) => {
                book.available = true;
                self.borrowed_books.pop();
                println!("{} returned {}", self.name, book.title);
            }
        }
    }
}
fn main() {
    let book1 = Book {
        title: String::from("The monk who sold his ferrari"),
        author: String::from("Robin Sharma"),
        isbn: String::from("1232334"),
        available: true,
    };

    let book2 = Book {
        title: String::from("The rust book"),
        author: String::from("Rust"),
        isbn: String::from("1232334"),
        available: true,
    };

    let book3 = Book {
        title: String::from("Zarahs deal"),
        author: String::from("Sharma"),
        isbn: String::from("1232334"),
        available: true,
    };

    let books = vec![book1, book2, book3];
}
