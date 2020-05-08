#![allow(dead_code)] // Code is only used within test module

use std::{
    collections::{HashMap, HashSet, VecDeque},
    iter::from_fn,
};

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
    pub fn dependencies_for(&self, src: &'a str) -> Option<impl Iterator<Item = &str>> {
        if !self.map.contains_key(src) {
            return None;
        };

        let mut frontier = VecDeque::new();
        let mut visited = HashSet::new();
        frontier.push_back(src);
        visited.insert(src);

        // Perform a bread-first search of the graph starting from `src`.
        // Each dependency need only be included in the result once. To achieve
        // this a set of visited nodes is maintained throughout the search;
        // a node is pushed onto the queue for searching iff it has not been
        // searched in a previous iteration. This also prevents infinite loops
        // in the case of cyclic dependencies.

        // TODO: Reduce nesting
        let deps_iter = from_fn(move || {
            if let Some(v) = frontier.pop_front() {
                if let Some(next_deps) = self.map.get(v) {
                    for dep in next_deps {
                        if !visited.contains(dep) {
                            visited.insert(dep);
                            frontier.push_back(dep);
                        }
                    }
                }
                Some(v)
            } else {
                None
            }
        });

        // Don't include `src` as the first item in the dependencies.
        Some(deps_iter.skip(1))
    }
}

#[cfg(test)]
mod tests {

    macro_rules! assert_ok_contents_eq {
        ($left:expr, $right:expr) => {{
            match ($left, $right) {
                (left_val, right_val) => {
                    let mut a: Vec<_> = left_val.unwrap().collect();
                    let mut b: Vec<_> = right_val.iter().cloned().collect();

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

        // let deps: Vec<_> = dep.dependencies_for("A").unwrap().collect();
        // assert_ok_contents_eq!(deps, ["B", "C", "E", "F", "G", "H"]);
        assert_ok_contents_eq!(dep.dependencies_for("A"), ["B", "C", "E", "F", "G", "H"]);
        assert_ok_contents_eq!(dep.dependencies_for("B"), ["C", "E", "F", "G", "H"]);
        assert_ok_contents_eq!(dep.dependencies_for("C"), ["G"]);
        assert_ok_contents_eq!(
            dep.dependencies_for("D"),
            ["A", "B", "C", "E", "F", "G", "H"]
        );
        assert_ok_contents_eq!(dep.dependencies_for("E"), ["F", "H"]);
        assert_ok_contents_eq!(dep.dependencies_for("F"), ["H"]);
    }

    #[test]
    fn test_cycle() {
        let mut dep = Dependencies::new();
        dep.add_direct("A", &["B"]);
        dep.add_direct("B", &["C"]);
        dep.add_direct("C", &["A"]);

        assert_ok_contents_eq!(dep.dependencies_for("A"), ["B", "C",]);
        assert_ok_contents_eq!(dep.dependencies_for("B"), ["A", "C"]);
        assert_ok_contents_eq!(dep.dependencies_for("C"), ["A", "B"]);
    }
}
