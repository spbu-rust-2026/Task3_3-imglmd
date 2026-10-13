use std::collections::HashSet;

pub fn unique_count(names: &[String]) -> usize {
    let set: HashSet<&String> = names.iter().collect();
    set.len()
}
