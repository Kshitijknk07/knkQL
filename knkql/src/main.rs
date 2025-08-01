mod query;

use query::{Query, handle_query};

fn main() {
    let input = "read user(123) { name email }";
    let result: Query = handle_query(input);
    dbg!(result);
}
