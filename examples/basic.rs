use serde_json::json;
use toon::{encode, Delimiter, EncodeOptions};

fn main() {
    println!("=== Basic Object ===");
    let data = json!({
        "id": 123,
        "name": "Ada",
        "active": true
    });
    println!("{}\n", encode(&data, None));

    println!("=== Nested Object ===");
    let data = json!({
        "user": {
            "id": 123,
            "name": "Ada",
            "tags": ["reading", "gaming"],
            "active": true
        }
    });
    println!("{}\n", encode(&data, None));

    println!("=== Array of Objects (Tabular) ===");
    let data = json!({
        "items": [
            { "sku": "A1", "qty": 2, "price": 9.99 },
            { "sku": "B2", "qty": 1, "price": 14.5 }
        ]
    });
    println!("{}\n", encode(&data, None));

    println!("=== GitHub Repositories Example ===");
    let data = json!({
        "repositories": [
            {
                "id": 28457823,
                "name": "freeCodeCamp",
                "stars": 430828,
                "forks": 42136
            },
            {
                "id": 132750724,
                "name": "build-your-own-x",
                "stars": 430102,
                "forks": 40388
            }
        ]
    });
    println!("{}\n", encode(&data, None));

    println!("=== With Tab Delimiter ===");
    let data = json!({
        "tags": ["reading", "gaming", "coding"]
    });
    let mut options = EncodeOptions::default();
    options.delimiter = Delimiter::Tab;
    println!("{}\n", encode(&data, Some(options)));

    println!("=== With Length Marker ===");
    let data = json!({
        "items": [
            { "id": 1, "name": "First" },
            { "id": 2, "name": "Second" }
        ]
    });
    let mut options = EncodeOptions::default();
    options.length_marker = Some('#');
    println!("{}\n", encode(&data, Some(options)));

    println!("=== Mixed Array ===");
    let data = json!({
        "items": [
            1,
            { "name": "object" },
            "text"
        ]
    });
    println!("{}", encode(&data, None));

    println!("\n=== Multi-Level Nested Object ===");
    let data = json!({
        "company": {
            "name": "TechCorp",
            "departments": {
                "engineering": {
                    "employees": [
                        {
                            "id": 1,
                            "name": "Alice",
                            "skills": ["rust", "python"],
                            "address": {
                                "city": "San Francisco",
                                "country": "USA"
                            }
                        },
                        {
                            "id": 2,
                            "name": "Bob",
                            "skills": ["javascript", "go"],
                            "address": {
                                "city": "Austin",
                                "country": "USA"
                            }
                        }
                    ],
                    "budget": {
                        "annual": 5000000,
                        "quarterly": {
                            "Q1": 1200000,
                            "Q2": 1300000,
                            "Q3": 1250000,
                            "Q4": 1250000
                        }
                    }
                },
                "marketing": {
                    "employees": [
                        {
                            "id": 3,
                            "name": "Carol",
                            "campaigns": ["social", "email"]
                        }
                    ]
                }
            }
        }
    });
    println!("{}", encode(&data, None));
}
