#![allow(dead_code)]

use serde_query::Deserialize;

const INPUT: &str = include_str!("./input.json");

#[test]
fn test_optional() {
    #[derive(Debug, Deserialize)]
    struct Author {
        #[query(".commit.author.name")]
        name: String,

        // does not exist
        #[query(".commit.author.city")]
        city: Option<String>,

        #[query(".commit.author.organizations.[].name")]
        orgs: Option<Vec<Option<String>>>,

        // does not exist
        #[query(".commit.author.teams.[].name")]
        teams: Option<Vec<Option<String>>>,
    }

    let author: Author = match serde_json::from_str::<Author>(INPUT) {
        Ok(a) => a,
        Err(e) => {
            println!("Error: {}", e);
            panic!("Unexpected error: {}", e)
        }
    };

    assert_eq!(author.name, "Maximilian Roos");
    assert_eq!(author.city, None);

    assert!(author.orgs.is_some());
    let orgs = author.orgs.as_ref().unwrap();
    assert_eq!(orgs.len(), 0);

    assert_eq!(author.teams, None);
}
