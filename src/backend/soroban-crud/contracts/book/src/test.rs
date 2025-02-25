#![cfg(test)]
use crate::{BookRecord, BookRecordClient};
use soroban_sdk::{Env, String};

#[test]
fn test_create_and_read() {
    // Initialize the environment
    let env = Env::default();
    let contract_id = env.register_contract(None, BookRecord);
    let client = BookRecordClient::new(&env, &contract_id);

    // Define book details
    let id = 1;
    let title = String::from_str(&env, "The Great Gatsby");
    let author = String::from_str(&env, "F. Scott Fitzgerald");
    let year = 1925;

    // Create a new book
    client.create(&id, &title, &author, &year);

    // Read the book by ID
    let book = client.read(&id).unwrap();

    // Assert that the book details are correct
    assert_eq!(book.title, title);
    assert_eq!(book.author, author);
    assert_eq!(book.year, year);
}

#[test]
fn test_update_book() {
    // Initialize the environment
    let env = Env::default();
    let contract_id = env.register_contract(None, BookRecord);
    let client = BookRecordClient::new(&env, &contract_id);

    // Define initial book details
    let id = 1;
    let title = String::from_str(&env, "The Great Gatsby");
    let author = String::from_str(&env, "F. Scott Fitzgerald");
    let year = 1925;

    // Create a new book
    client.create(&id, &title, &author, &year);

    // Update the book details
    let updated_title = String::from_str(&env, "Great Gatsby (Updated)");
    let updated_author = String::from_str(&env, "F. S. Fitzgerald");
    let updated_year = 1926;

    client.update(&id, &updated_title, &updated_author, &updated_year);

    // Read the updated book
    let book = client.read(&id).unwrap();

    // Assert that the updated details are correct
    assert_eq!(book.title, updated_title);
    assert_eq!(book.author, updated_author);
    assert_eq!(book.year, updated_year);
}

#[test]
fn test_delete_book() {
    // Initialize the environment
    let env = Env::default();
    let contract_id = env.register_contract(None, BookRecord);
    let client = BookRecordClient::new(&env, &contract_id);

    // Define book details
    let id = 1;
    let title = String::from_str(&env, "The Great Gatsby");
    let author = String::from_str(&env, "F. Scott Fitzgerald");
    let year = 1925;

    // Create a new book
    client.create(&id, &title, &author, &year);

    // Delete the book
    client.delete(&id);

    // Try to read the deleted book
    let book = client.read(&id);

    // Assert that the book is no longer present
    assert!(book.is_none());
}

#[test]
#[should_panic(expected = "Book with this ID already exists")]
fn test_create_duplicate_book() {
    // Initialize the environment
    let env = Env::default();
    let contract_id = env.register_contract(None, BookRecord);
    let client = BookRecordClient::new(&env, &contract_id);

    // Define book details
    let id = 1;
    let title = String::from_str(&env, "The Great Gatsby");
    let author = String::from_str(&env, "F. Scott Fitzgerald");
    let year = 1925;

    // Create the same book twice
    client.create(&id, &title, &author, &year);
    client.create(&id, &title, &author, &year); // This should panic
}

#[test]
#[should_panic(expected = "Book with this ID does not exist")]
fn test_update_nonexistent_book() {
    // Initialize the environment
    let env = Env::default();
    let contract_id = env.register_contract(None, BookRecord);
    let client = BookRecordClient::new(&env, &contract_id);

    // Attempt to update a book that doesn't exist
    let id = 1;
    let updated_title = String::from_str(&env, "Great Gatsby (Updated)");
    let updated_author = String::from_str(&env, "F. S. Fitzgerald");
    let updated_year = 1926;

    client.update(&id, &updated_title, &updated_author, &updated_year); // This should panic
}

#[test]
#[should_panic(expected = "Book with this ID does not exist")]
fn test_delete_nonexistent_book() {
    // Initialize the environment
    let env = Env::default();
    let contract_id = env.register_contract(None, BookRecord);
    let client = BookRecordClient::new(&env, &contract_id);

    // Attempt to delete a book that doesn't exist
    let id = 1;
    client.delete(&id); // This should panic
}
