#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Bytes, Env, FromVal, Map, String, Symbol,
};

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Book {
    pub title: Bytes,
    pub author: Bytes,
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
    fn string_to_bytes(env: &Env, s: String) -> Bytes {
        // Check if the string exceeds the maximum length
        if s.len() > Self::MAX_STRING_LENGTH {
            panic!("String exceeds maximum length of 256 characters");
        }

        // Bytes::from_slice(env, &s.to_bytes())
        // Convert the String to a byte vector and then to Bytes
        // Bytes::from_slice(env, &s.to_vec())
        Bytes::from_val(env, &s.to_val())
    }

    // Helper function to convert Bytes to string
    fn bytes_to_string(env: &Env, bytes: Bytes) -> String {
        // Convert Bytes to a UTF-8 String
        // String::from_bytes(env, &bytes).unwrap_or_else(|_| panic!("Invalid UTF-8 data"))
        // String::from_bytes(env, &bytes.to_vec()).unwrap_or_else(|_| panic!("Invalid UTF-8 data"))
        String::from_val(env, &bytes.to_val())
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

        let title_bytes = Self::string_to_bytes(&env, title);
        let author_bytes = Self::string_to_bytes(&env, author);

        let book = Book {
            title: title_bytes,
            author: author_bytes,
            year,
        };
        books.set(id, book);
        env.storage().persistent().set(&Self::BOOKS, &books);
    }

    // Read a book by ID
    pub fn read(env: Env, id: u32) -> Option<(String, String, u32)> {
        let books = env
            .storage()
            .persistent()
            .get::<Symbol, Map<u32, Book>>(&Self::BOOKS)
            .unwrap_or_else(|| Map::new(&env));

        books.get(id).map(|book| {
            (
                Self::bytes_to_string(&env, book.title),
                Self::bytes_to_string(&env, book.author),
                book.year,
            )
        })
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

        let title_bytes = Self::string_to_bytes(&env, title);
        let author_bytes = Self::string_to_bytes(&env, author);

        let updated_book = Book {
            title: title_bytes,
            author: author_bytes,
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
