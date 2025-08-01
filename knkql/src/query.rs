#[allow(dead_code)]
#[derive(Debug)]

pub struct Query {
    pub action: String,
    pub whom: String,
    pub id: i32,
    pub fields: Vec<String>,
}

pub fn handle_query(query: &str) -> Query {
    // Here we are Removing the "read" part of our logic
    let without_keyword = query.strip_prefix("read ").unwrap_or(query);

    // Here we are splitting at  '(' to isolate the "user"
    let parts: Vec<&str> = without_keyword.split('(').collect();
    let whom = parts[0].trim();

    // Here we are trying to Extract ID from second part
    let after_paren = parts[1];
    let id_part: Vec<&str> = after_paren.split(')').collect();
    let id_str = id_part[0].trim();
    let id: i32 = id_str.parse().unwrap_or(0);

    // Here we are now Extracting Fields from {...}
    let start = query.find('{').unwrap_or(0);
    let end = query.find('}').unwrap_or(0);
    let inside_braces = &query[start + 1..end];

    // Here we are Splitting by Whitespaces and Collecting it
    let fields: Vec<String> = inside_braces
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    Query {
        action: "read".to_string(),
        whom: whom.to_string(),
        id,
        fields,
    }
}
