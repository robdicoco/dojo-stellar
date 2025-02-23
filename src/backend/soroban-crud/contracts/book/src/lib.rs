#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Env, IntoVal, Map, Symbol, TryFromVal, Val,
    Vec,
};

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Book {
    pub title: Symbol,
    pub author: Symbol,
    pub year: u32,
}

impl TryFromVal<Env, Val> for Book {
    type Error = ();

    fn try_from_val(env: &Env, val: &Val) -> Result<Self, Self::Error> {
        Ok(Book {
            title: Symbol::try_from_val(env, &val.get_tagged(0)?).unwrap(),
            author: Symbol::try_from_val(env, &val.get_tagged(1)?).unwrap(),
            year: u32::try_from_val(env, &val.get_tagged(2)?).unwrap(),
        })
    }
}

impl IntoVal<Env, Val> for Book {
    fn into_val(&self, env: &Env) -> Val {
        let mut vec = Vec::new(env);
        vec.push(self.title.into_val(env));
        vec.push(self.author.into_val(env));
        vec.push(self.year.into_val(env));
        vec.into_val(env)
    }
}

#[contract]
pub struct BookRecord;

#[contractimpl]
impl BookRecord {
    // Storage key for the map of books
    const BOOKS: Symbol = symbol_short!("BOOKS");

    // Create a new book
    pub fn create(env: Env, id: u32, title: Symbol, author: Symbol, year: u32) {
        let mut books = env
            .storage()
            .persistent()
            .get::<Symbol, Map<u32, Book>>(Self::BOOKS)
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
        env.storage().persistent().set(Self::BOOKS, &books);
    }

    // Read a book by ID
    pub fn read(env: Env, id: u32) -> Option<Book> {
        let books = env
            .storage()
            .persistent()
            .get::<Symbol, Map<u32, Book>>(Self::BOOKS)
            .unwrap_or_else(|| Map::new(&env));
        books.get(id)
    }

    // Update an existing book
    pub fn update(env: Env, id: u32, title: Symbol, author: Symbol, year: u32) {
        let mut books = env
            .storage()
            .persistent()
            .get::<Symbol, Map<u32, Book>>(Self::BOOKS)
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
        env.storage().persistent().set(Self::BOOKS, &books);
    }

    // Delete a book by ID
    pub fn delete(env: Env, id: u32) {
        let mut books = env
            .storage()
            .persistent()
            .get::<Symbol, Map<u32, Book>>(Self::BOOKS)
            .unwrap_or_else(|| Map::new(&env));

        if !books.contains_key(id) {
            panic!("Book with this ID does not exist");
        }

        books.remove(id);
        env.storage().persistent().set(Self::BOOKS, &books);
    }
}

mod test;
