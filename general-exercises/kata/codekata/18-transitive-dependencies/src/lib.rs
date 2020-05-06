#![allow(dead_code)] // Code is only used within test module

use std::collections::{HashMap, HashSet, VecDeque};

struct Dependencies<'a> {
    map: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> Dependencies<'a> {
    pub fn new() -> Self {
        Dependencies {
            map: HashMap::new(),
        }
    }

    pub fn add_direct(&mut self, src: &'a str, deps: &[&'a str]) -> &mut Self {
        self.map.insert(src, deps.into());
        self
    }

    /// Find the transitive dependencies of `src`. Returns `None` if `src` is
    /// not present in the dependency graph. This is *not* a topological
    /// ordering; cyclic dependencies are handled as only the *existence* of
    /// the dependency is required, not some resolution order.
    // TODO: Re-implement using Iterator
    pub fn dependencies_for(&self, src: &'a str) -> Option<Vec<&'a str>> {
        if !self.map.contains_key(src) {
            return None;
        };

        // Perform a bread-first search of the graph starting from `src`.
        // Each dependency need only be included in the result once. To achieve
        // this a set of visited nodes is maintained throughout the search;
        // a node is pushed onto the queue for searching iff it has not been
        // searched in a previous iteration. This also prevents infinite loops
        // in the case of cyclic dependencies.

        let mut deps = Vec::new();
        let mut visited = HashSet::new();
        let mut frontier = VecDeque::new();

        // Start the search at `src`
        visited.insert(src);
        frontier.push_back(src);

        // TODO: Reduce nesting
        while let Some(v) = frontier.pop_front() {
            if let Some(next_deps) = self.map.get(v) {
                for dep in next_deps {
                    if !visited.contains(dep) {
                        visited.insert(dep);
                        frontier.push_back(dep);
                    }
                }
            }
            // TODO: Find a way to avoid this. Perhaps implement this with an
            // Iterator instead of returning a Vec and then skip(1)?
            if v != src {
                deps.push(v);
            }
        }
        Some(deps)
    }
}

#[cfg(test)]
mod tests {

    macro_rules! assert_contents_eq {
        ($left:expr, $right:expr) => {{
            match (&$left, &$right) {
                (left_val, right_val) => {
                    let mut a: Vec<_> = left_val.iter().collect();
                    let mut b: Vec<_> = right_val.iter().collect();
                    a.sort();
                    b.sort();

                    assert_eq!(a, b);
                }
            }
        }};
    }

    use super::*;

    // Test cases given in the Kata are alphabetically ordered. I have chosen
    // to ignore this here and test based on the *set* of dependencies, not
    // their ordering. I feel this better represents the problem and removes
    // the additional overhead of performing a sort on the result; aiding with
    // the follow up question on scaling to a large input.
    #[test]
    fn test_basic() {
        let mut dep = Dependencies::new();
        dep.add_direct("A", &["B", "C"]);
        dep.add_direct("B", &["C", "E"]);
        dep.add_direct("C", &["G"]);
        dep.add_direct("D", &["A", "F"]);
        dep.add_direct("E", &["F"]);
        dep.add_direct("F", &["H"]);

        assert_contents_eq!(
            dep.dependencies_for("A").unwrap(),
            ["B", "C", "E", "F", "G", "H"]
        );
        assert_contents_eq!(
            dep.dependencies_for("B").unwrap(),
            ["C", "E", "F", "G", "H"]
        );
        assert_contents_eq!(dep.dependencies_for("C").unwrap(), ["G"]);
        assert_contents_eq!(
            dep.dependencies_for("D").unwrap(),
            ["A", "B", "C", "E", "F", "G", "H"]
        );
        assert_contents_eq!(dep.dependencies_for("E").unwrap(), ["F", "H"]);
        assert_contents_eq!(dep.dependencies_for("F").unwrap(), ["H"]);
    }

    #[test]
    fn test_cycle() {
        let mut dep = Dependencies::new();
        dep.add_direct("A", &["B"]);
        dep.add_direct("B", &["C"]);
        dep.add_direct("C", &["A"]);

        assert_contents_eq!(dep.dependencies_for("A").unwrap(), ["B", "C",]);
        assert_contents_eq!(dep.dependencies_for("B").unwrap(), ["A", "C"]);
        assert_contents_eq!(dep.dependencies_for("C").unwrap(), ["A", "B"]);
    }
}
