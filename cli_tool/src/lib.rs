pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            res.push(line);
        }
    }
    res
}

pub fn case_sensitive_search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut res = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            res.push(line);
        }
    }
    res
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn fail_test() {
        let query = "duck";
        let contents = "\
            hello world from duck,
            this is the world
        ";

        assert_eq!(search(query, contents), vec!["hello world from duck,"]);
    }
}
