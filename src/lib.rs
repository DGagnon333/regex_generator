use std::collections::HashSet;

/// Node structure representing a regex pattern in the tree.
#[derive(Debug, Clone)]
struct Node {
    pattern: String,
    children: Vec<Node>,
}

/// Generates a common regex pattern tree that matches all the given strings.
///
/// # Arguments
///
/// * `strings` - A slice of strings to generate the regex pattern tree from.
///
/// # Returns
///
/// The root node of the regex pattern tree.
pub fn generate_regex_tree(strings: &[&str]) -> Node {
    if strings.is_empty() {
        return Node {
            pattern: String::new(),
            children: vec![],
        };
    }

    let substrings = find_common_substrings(strings);
    build_tree(&substrings)
}

/// Finds common substrings among the given strings.
///
/// # Arguments
///
/// * `strings` - A slice of strings to find common substrings.
///
/// # Returns
///
/// A vector of common substrings.
fn find_common_substrings(strings: &[&str]) -> Vec<String> {
    let mut common_substrings = HashSet::new();
    let first = strings[0];

    for i in 0..first.len() {
        for j in i + 1..=first.len() {
            let substring = &first[i..j];
            if strings.iter().all(|s| s.contains(substring)) {
                common_substrings.insert(substring.to_string());
            }
        }
    }

    let mut substrings: Vec<String> = common_substrings.into_iter().collect();
    substrings.sort_by_key(|s| s.len());
    substrings.reverse();
    substrings
}

/// Builds a tree of regex patterns from the common substrings.
///
/// # Arguments
///
/// * `substrings` - A vector of common substrings to build the tree from.
///
/// # Returns
///
/// The root node of the regex pattern tree.
fn build_tree(substrings: &[String]) -> Node {
    let root = Node {
        pattern: ".*".to_string(),
        children: vec![],
    };

    let mut nodes: Vec<Node> = vec![root.clone()];

    for substring in substrings {
        let new_node = Node {
            pattern: format!(".*{}.*", regex::escape(substring)),
            children: vec![],
        };
        nodes.push(new_node);
    }

    let mut root = nodes.remove(0);

    for i in 0..nodes.len() {
        let mut children = vec![];
        for j in i + 1..nodes.len() {
            if nodes[j].pattern.contains(&nodes[i].pattern) {
                children.push(nodes[j].clone());
            }
        }
        nodes[i].children = children;
    }

    root.children = nodes;
    root
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_regex_tree() {
        let inputs = vec![
            "highlighted text",
            "highlighted part",
            "highlighted section",
        ];
        let tree = generate_regex_tree(&inputs);
        assert_eq!(tree.pattern, ".*");
        assert!(tree
            .children
            .iter()
            .any(|n| n.pattern == ".*highlighted .*"));
    }

    #[test]
    fn test_find_common_substrings() {
        let inputs = vec![
            "highlighted text",
            "highlighted part",
            "highlighted section",
        ];
        let common_substrings = find_common_substrings(&inputs);
        assert!(common_substrings.contains(&"highlighted ".to_string()));
    }
}
