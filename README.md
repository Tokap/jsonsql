# jsonsql

A MySQL abstraction for basic select and insert interactions using json as the primary means of information transfer.

> **NOTE** This is an alpha build and will experience rapid change.

## Setup:

Update dependencies:
```rust
// Cargo.toml
[dependencies]
jsonsql = {version = "*", git = "https://github.com/Tokap/jsonsql" }
```
Run: `cargo build`

Update main file:
```rust
// main.rs
`extern crate jsonsql;`
```

## Quick Summary:
This library is broken into modules based on individual purpose. The modules currently include:
  - pool
  - read
  - write

More modules will be added as functionality expands.  

## Setup Pool Connection:
Different functions are provided for Pool construction. A user may input the required fields individually or as a JSON string based on the fn being used.
The example below shows an itemized Pool creation. If a field is not required/available, a "" may be used instead. (except for the port number, where a 0 should be used in place of a value)

```rust
use jsonsql::pool::{Pool, build_basic_pool};

let simple_pool: Pool = build_basic_pool("some_hostname", "my_database", "user", "password", 3306);
```

If a user has a String of JSON data, they may pass it in as an argument to build_pool_json. This function is flexible and permits the user to omit key/value information for options not being used.
The list of available options for Pool config when passing them through JSON are:
- hostname
- port
- db
- user
- password
- socket

```rust
use jsonsql::pool::{Pool, build_basic_pool};

let json_string: String = String::from(
                          r#" { "hostname": "127.0.0.1",
                                "db": "my_database",
                                "user": "some_user",
                                "password": "mediocre_password"  
                              } "#);

let pool_from_json: Pool = build_pool_json(json_string);
```

> **NOTE** Although a user can construct a JSON string manually as shown above, there are multiple rust crates that make JSON creation simple and allow you to easily manipulate the results.

## Read Information:

There are currently 4 methods to read from the Database using this library. They are:
- `get_by_param` -> takes 4 parameters: a search key (i.e 'name'), a search value (i.e. 'bob'), and a table as &str + a pool connection.
- `get_by_two_params` -> takes 4 parameters: two key/value tuples of &str, a table as &str + a pool connection.
- `get_json_by_id` -> a common query request. It takes 3 parameters - the id and table as &str + a pool connection.
- `get_by_raw` -> takes a raw MySQL Select statement as a String + a pool connection. Executes the raw statement assuming proper syntax.

**Get By Param:**
```rust
use jsonsql::pool::{Pool, build_basic_pool};
use jsonsql::read::{get_by_param};


let simple_pool: Pool = build_basic_pool("some_hostname", "my_database", "user", "password", 3306);
let return_value: Result<String, String> = get_by_param("name", "bob", "account_data", simple_pool);

println!("My Outcome Looks Like: {}", return_value.unwrap());
// "[{"id":"1","name":"bob","address":"123 Front Street"}]"
```

**Get By Two Params:**
```rust
use jsonsql::pool::{Pool, build_basic_pool};
use jsonsql::read::{get_by_two_params};

let simple_pool: Pool = build_basic_pool("some_hostname", "my_database", "user", "password", 3306);
let return_value: Result<String, String> = get_by_two_params(("name", "bob"),("id", "1"), "account_data", simple_pool);

println!("My Outcome Looks Like: {}", return_value.unwrap());`
// "[{"id":"1","name":"bob","address":"123 Front Street"}]"
```

**Get By Id:**
```rust
use jsonsql::pool::{Pool, build_basic_pool};
use jsonsql::read::{get_json_by_id};

let simple_pool: Pool = build_basic_pool("some_hostname", "my_database", "user", "password", 3306);
let return_value: Result<String, String> = get_json_by_id("2", "account_data", simple_pool);

println!("My Outcome Looks Like: {}", return_value.unwrap());`
// "[{"id":"2","name":"jerry","address":"456 Front Street"}]"
```

**Raw Query:**
```rust
use jsonsql::pool::{Pool, build_basic_pool};
use jsonsql::read::{get_by_raw};

let simple_pool: Pool = build_basic_pool("some_hostname", "my_database", "user", "password", 3306);
let sql: String = "SELECT * FROM ..."

let return_value: Result<String, String> = get_by_raw(sql, simple_pool);

println!("My Outcome Looks Like: {}", return_value.unwrap());
// "[{"id":"1","name":"bob","address":"123 Front Street"}, {"id":"2","name":"jerry","address":"456 Front Street"}]"
```

> **NOTE** All queries return Results that must be unwrapped and may contain an error if the query process failed at any point.


## Write Information:

There are currently 3 primary methods to write to a Database using this library. They are:
- `vec_write_to_table` -> takes a Vector of Tuples (String, String) containing key/value pairs, a table name as a String + a pool connection. Tuples should include all required fields for an insert statement.
- `json_write_to_table` -> takes a JSON String containing key/value pairs with the same requirement as above, a table name as a String + a pool connection.
- `raw_write_to_table` -> takes a raw MySQL Insert statement as a String + a pool connection. Executes the raw statement assuming proper syntax.

**Using a Vec to Write to Table:**
```rust
use jsonsql::pool::{Pool, build_basic_pool};
use jsonsql::write::{vec_write_to_table};

let mut tuple_vec: Vec<(String, String)> = Vec::new();
    tuple_vec.push(("first_name".to_string(), "bob".to_string()));
    tuple_vec.push(("last_name".to_string(), "smith".to_string()));
    tuple_vec.push(("age".to_string(), "25".to_string()));

let table: String = String::from("account_data");
let simple_pool: Pool = build_basic_pool("some_hostname", "my_database", "user", "password", 3306);
let return_value: Result<SqlWriteReturn, String> = vec_write_to_table(tuple_vec, table, simple_pool);

println!("My Confirmation Data Looks Like: {}", return_value.unwrap());
```

**Using JSON to Write to Table:**
```rust
use jsonsql::pool::{Pool, build_basic_pool};
use jsonsql::write::{json_write_to_table};

let table: String = String::from("account_data");
let json_string: String = String::from(
                                      r#" {
                                      "first_name": "bob",
                                      "last_name": "smith",
                                      "age": "25" } "#);

let simple_pool: Pool = build_basic_pool("some_hostname", "my_database", "user", "password", 3306);
let return_value: Result<SqlWriteReturn, String> = json_write_to_table(json_string, table, simple_pool);

println!("My Confirmation Data Looks Like: {}", return_value.unwrap());
```

**Raw Query:**
```rust
use jsonsql::pool::{Pool, build_basic_pool};
use jsonsql::write::{raw_write_to_table};

let simple_pool: Pool = build_basic_pool("some_hostname", "my_database", "user", "password", 3306);
let sql: String = "INSERT INTO ... VALUES ..."

let return_value: Result<SqlWriteReturn, String> = raw_write_to_table(sql, simple_pool);

println!("My Confirmation Data Looks Like: {}", return_value.unwrap());
```

> **NOTE** All insert statements return Results containing confirmation details or an error if the insert process failed at any point.
