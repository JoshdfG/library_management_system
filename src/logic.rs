#[derive(Clone, Debug)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub available: bool,
}

pub enum Role {
    Member,
    Librarian,
}

#[derive(PartialEq)]
pub enum BookType {
    Fiction,
    NonFiction,
    Reference,
    Magazine,
}

pub struct User {
    pub name: String,
    pub role: Role,
    pub borrowed_books: Vec<Book>,
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
    pub fn borrow_book(&mut self, book: &mut Book, book_type: BookType) {
        match self.role {
            Role::Member => {
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
            Role::Librarian => {
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

    pub fn show_borrowed_books(&self) -> Vec<Book> {
        self.borrowed_books.to_owned()
    }

    //this is an associated function as it doesn't make reference to self and cannot be
    // allowed to access self details because it doesn't take self as an argument!
    pub fn show_borrowed_book() -> String {
        String::from("Hello world!")
    }

    pub fn return_book(&mut self, book: &mut Book, book_type: BookType) {
        match self.role {
            Role::Member => {
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
            Role::Librarian => {
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
