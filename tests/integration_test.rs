use nexum_core::{StorageEngine, Parser, Executor};

#[test]
fn test_semantic_cache_integration() {
    let storage = StorageEngine::memory().unwrap();
    let executor = Executor::new(storage).with_cache();
    
    let create_sql = "CREATE TABLE products (id INTEGER, name TEXT, price INTEGER)";
    let create_stmt = Parser::parse(create_sql).unwrap();
    executor.execute(create_stmt).unwrap();
    
    let insert_sql = "INSERT INTO products (id, name, price) VALUES (1, 'Laptop', 1000), (2, 'Mouse', 25), (3, 'Keyboard', 75)";
    let insert_stmt = Parser::parse(insert_sql).unwrap();
    let result = executor.execute(insert_stmt).unwrap();
    
    println!("First INSERT: {:?}", result);
    
    let select_sql = "SELECT * FROM products";
    let select_stmt = Parser::parse(select_sql).unwrap();
    
    println!("\nFirst SELECT (should miss cache):");
    let result1 = executor.execute(select_stmt.clone()).unwrap();
    println!("{:?}", result1);
    
    println!("\nSecond SELECT (should hit cache):");
    let result2 = executor.execute(select_stmt).unwrap();
    println!("{:?}", result2);
}

#[test]
fn test_full_sql_workflow() {
    let storage = StorageEngine::memory().unwrap();
    let executor = Executor::new(storage);
    
    let create = Parser::parse("CREATE TABLE users (id INTEGER, name TEXT)").unwrap();
    executor.execute(create).unwrap();
    
    let insert = Parser::parse("INSERT INTO users (id, name) VALUES (1, 'Alice'), (2, 'Bob')").unwrap();
    let result = executor.execute(insert).unwrap();
    
    match result {
        nexum_core::executor::ExecutionResult::Inserted { rows, .. } => {
            assert_eq!(rows, 2);
        }
        _ => panic!("Expected Inserted result"),
    }
    
    let select = Parser::parse("SELECT * FROM users").unwrap();
    let result = executor.execute(select).unwrap();
    
    match result {
        nexum_core::executor::ExecutionResult::Selected { rows, .. } => {
            assert_eq!(rows.len(), 2);
        }
        _ => panic!("Expected Selected result"),
    }
}
