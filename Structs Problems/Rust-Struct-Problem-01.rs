#[derive(Debug)]
enum BookStatus {
    Available,
    Borrowed,
    Overdue,
}

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    year: u64,
    status: BookStatus,
}

fn main() {
    let mut book_one = Book {
        title: String::from("Maths"),
        author: String::from("Jack"),
        year: 2001,
        status: BookStatus::Available,
    };
    let mut book_two = Book {
        title: String::from("History"),
        author: String::from("Ben"),
        year: 2010,
        status: BookStatus::Borrowed,
    };
    let book_three = Book {
        title: String::from("Science"),
        author: String::from("Jhon"),
        year: 2011,
        status: BookStatus::Overdue,
    };

    borrow_book(&mut book_one);
    return_book(&mut book_two);
    is_overdue(&book_three);
}

fn borrow_book(book: &mut Book) {
    let can_borrow: bool;
    match book.status {
        BookStatus::Available => can_borrow = true,
        BookStatus::Borrowed => can_borrow = false,
        BookStatus::Overdue => can_borrow = false,
    }

    if can_borrow {
        book.status = BookStatus::Borrowed;
        print!("You just borrowed {} book", book.title);
    } else {
        print!("{} book is not available", book.title);
    }
}

fn return_book(book: &mut Book) {
    book.status = BookStatus::Available;
}

fn is_overdue(book: &Book) -> bool {
    match book.status {
        BookStatus::Available => return false,
        BookStatus::Borrowed => return false,
        BookStatus::Overdue => return true,
    }
}
