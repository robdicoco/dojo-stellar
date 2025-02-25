#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, Map, String, Symbol};

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Book {
    pub title: String,
    pub author: String,
    pub year: u32,
}

#[contract]
pub struct BookRecord;

#[contractimpl]
impl BookRecord {
    // Storage key for the map of books
    const BOOKS: Symbol = symbol_short!("BOOKS");

    // Maximum allowed length for title and author (256 UTF-8 characters)
    const MAX_STRING_LENGTH: u32 = 256;

    // Helper function to convert string to Bytes with length check
    fn validate_string(s: String) -> String {
        // Check if the string exceeds the maximum length
        if s.len() > Self::MAX_STRING_LENGTH {
            panic!("String exceeds maximum length of 256 characters");
        }
        s
    }

    // Create a new book
    pub fn create(env: Env, id: u32, title: String, author: String, year: u32) {
        let mut books = env
            .storage()
            .persistent()
            .get::<Symbol, Map<u32, Book>>(&Self::BOOKS)
            .unwrap_or_else(|| Map::new(&env));
        if books.contains_key(id) {
            panic!("Book with this ID already exists");
        }
        let title_validated = Self::validate_string(title);
        let author_validated = Self::validate_string(author);

        let book = Book {
            title: title_validated,
            author: author_validated,
            year,
        };
        books.set(id, book);
        env.storage().persistent().set(&Self::BOOKS, &books);
    }

    // Read a book by ID
    pub fn read(env: Env, id: u32) -> Option<Book> {
        let books = env
            .storage()
            .persistent()
            .get::<Symbol, Map<u32, Book>>(&Self::BOOKS)
            .unwrap_or_else(|| Map::new(&env));
        books.get(id)
    }

    // Update an existing book
    pub fn update(env: Env, id: u32, title: String, author: String, year: u32) {
        let mut books = env
            .storage()
            .persistent()
            .get::<Symbol, Map<u32, Book>>(&Self::BOOKS)
            .unwrap_or_else(|| Map::new(&env));
        if !books.contains_key(id) {
            panic!("Book with this ID does not exist");
        }

        let title_validated = Self::validate_string(title);
        let author_validated = Self::validate_string(author);

        let updated_book = Book {
            title: title_validated,
            author: author_validated,
            year,
        };
        books.set(id, updated_book);
        env.storage().persistent().set(&Self::BOOKS, &books);
    }

    // Delete a book by ID
    pub fn delete(env: Env, id: u32) {
        let mut books = env
            .storage()
            .persistent()
            .get::<Symbol, Map<u32, Book>>(&Self::BOOKS)
            .unwrap_or_else(|| Map::new(&env));
        if !books.contains_key(id) {
            panic!("Book with this ID does not exist");
        }
        books.remove(id);
        env.storage().persistent().set(&Self::BOOKS, &books);
    }
}

mod test;
