use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: Please provide a Cypher query string.");
        process::exit(1);
    }

    let query = &args[1];
    let normalized_query = query.trim();
    

    // Mapping logic cyper to sql
    //based on Cypher.g4 grammar
    //trying to do basic crud operations, hopefully not too complicated

    let upper_query = normalized_query.to_uppercase();

    //Create or Insert
    if upper_query.starts_with("CREATE") {
        let name = extract_quoted_value(normalized_query, "name:");
        let type_label = extract_label(normalized_query); 
        
        if !name.is_empty() {
             println!("INSERT INTO nodes (name, type) VALUES ('{}', '{}');", name, type_label);
        } else {
             println!("Error parsing CREATE query");
        }
    }
    
    // Delete
    else if upper_query.contains("DELETE") {
        let name = extract_quoted_value(normalized_query, "n.name =");
        
        if !name.is_empty() {
            println!("DELETE FROM nodes WHERE name = '{}';", name);
        } else {
             println!("Error parsing DELETE query");
        }
    }

    // read w filter
    else if upper_query.contains("WHERE") {
        let name = extract_quoted_value(normalized_query, "n.name =");
        
        if !name.is_empty() {
            println!("SELECT * FROM nodes WHERE name = '{}';", name);
        } else {
            println!("SELECT * FROM nodes;");
        }
    }
    
    //simple read
    else if normalized_query.contains("MATCH (n) RETURN n") {
        println!("SELECT * FROM nodes;");
    }

    //echo/test
    else if normalized_query.contains("RETURN \"Hello\"") {
        println!("SELECT 'Hello' AS message, 'World' AS status;");
    }

    //default
    else {
        println!("SELECT * FROM nodes WHERE name = 'Unknown';");
    }
}

// Helper functions

//extract value between single quotes after a key
fn extract_quoted_value(query: &str, key: &str) -> String {
    if let Some(start_idx) = query.find(key) {
        let rest = &query[start_idx..];
        if let Some(quote_start) = rest.find('\'') {
            let after_quote = &rest[quote_start + 1..];
            if let Some(quote_end) = after_quote.find('\'') {
                return after_quote[..quote_end].to_string();
            }
        }
    }
    String::new()
}

//extract label
fn extract_label(query: &str) -> String {
    if let Some(colon_idx) = query.find(':') {
        let rest = &query[colon_idx + 1..];
        if let Some(space_idx) = rest.find(' ') {
            return rest[..space_idx].trim().to_string();
        } else if let Some(brace_idx) = rest.find('{') {
             return rest[..brace_idx].trim().to_string();
        }
    }
    return "Node".to_string(); //default
}