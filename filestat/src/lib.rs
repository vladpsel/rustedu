use std::{collections::HashMap, fs};

pub struct Config {
    pub filepath: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() <= 1 || args.len() > 2 {
            return Err("Invalid arguments");
        }

        Ok(Config {
            filepath: args[1].clone(),
        })
    }

    pub fn analize(&self) {
        let mut word_count_map: HashMap<String, i32> = HashMap::new();

        let rows = fs::read_to_string(&self.filepath).expect("Error open the file");
        let words: Vec<&str> = rows.split_whitespace().collect();

        let lines: usize = rows.lines().count();
        let chars = rows.chars().count();

        for word in &words {
            //chars += word.trim_matches(&[',', '.'][..]).chars().count();
            *word_count_map.entry(word.to_lowercase()).or_insert(0) += 1;
        }

        let uniqs = word_count_map.len();
        let mut top: Vec<_> = word_count_map.iter().collect();
        top.retain(|x| x.0.len() > 4);
        top.sort_by(|a, b| b.1.cmp(&a.1));

        let top_result = top
            .iter()
            .take(5)
            .map(|(k, v)| format!("{}({})", k, v))
            .collect::<Vec<_>>()
            .join(", ");

        println!("Lines: {lines}");
        println!("Words: {}", words.len());
        println!("Chars: {chars}");
        println!("Unique words: {uniqs}");
        println!("Top 5 words: {top_result}");
    }
}
