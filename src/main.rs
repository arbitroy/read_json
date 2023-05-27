use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Paragraph{
    name : String
}
#[derive(Serialize, Deserialize)]
struct Article {
    article : String,
    author : String,
    paragraph : Vec<Paragraph>
}

fn main() {
    let json: &str = r#"
    {
        "article" : "How to work with json in rust",
        "author" : "Austine Ndauwa",
        "paragraph" : [
            {
            "name" : "Title text"
            },
            {
                "name" : "Description"
            },
            {
                "name" : "Content: The Best Rust programmer ever \"Austine Waweru Ndauwa\""
            }
            ]
        }"#;
        let parsed: Article = read_json_typed(json);
        println!("\n\n The name of the first paragraph is : {}", parsed.paragraph[2].name);
}

fn read_json_typed(raw_json: &str) -> Article {
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    return parsed;
}
