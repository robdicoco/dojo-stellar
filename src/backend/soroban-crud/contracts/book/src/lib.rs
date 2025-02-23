#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Bytes, Env, Map, Symbol};

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

    // Create a new book
    pub fn create(env: Env, id: u32, title: Bytes, author: Bytes, year: u32) {
        let mut books = env
            .storage()
            .persistent()
            .get::<Symbol, Map<u32, Book>>(&Self::BOOKS)
            .unwrap_or_else(|| Map::new(&env));

        if books.contains_key(id) {
            panic!("Book with this ID already exists");
        }

        let book = Book {
            title,
            author,
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
    pub fn update(env: Env, id: u32, title: Bytes, author: Bytes, year: u32) {
        let mut books = env
            .storage()
            .persistent()
            .get::<Symbol, Map<u32, Book>>(&Self::BOOKS)
            .unwrap_or_else(|| Map::new(&env));

        if !books.contains_key(id) {
            panic!("Book with this ID does not exist");
        }

        let updated_book = Book {
            title,
            author,
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
