/*!
...
*/

use std::{
    collections::HashMap,
    iter::Enumerate,
    mem,
    slice::Iter,
};

use crate::analyse::{
    combinatoric::Partition,
    DependencyGraph,
};

type Look = HashMap<usize, Vec<(usize, usize)>>;
type Deps = Vec<HashMap<usize, Vec<usize>>>;

#[derive(Debug, Clone)]
pub struct TimeGraph {
    first: Partition<Vec<usize>>,
    deps: Deps,
    look: Look,
}

impl From<DependencyGraph> for TimeGraph {
    fn from(mut graph: DependencyGraph) -> Self {
        fn resolve<'l>(
            this_layer: impl Iterator<Item = (&'l usize, &'l Vec<usize>)>,
            rest: &Enumerate<Iter<'_, HashMap<usize, Vec<usize>>>>,
            look: &mut Look,
        ) {
            for (this_bit, dependencies) in this_layer {
                let mut dependents = Vec::new();
                for (mut layer_idx, layer) in rest.clone() {
                    for (bit, deps) in layer.iter() {
                        if let Some(position) = deps.iter().position(|b| b == this_bit)
                        {
                            dependents.push((layer_idx, *bit));
                        }
                    }
                }
                look.insert(*this_bit, dependents);
            }
        }

        let mut graph = graph.into_iter();
        let first = graph.next().unwrap();
        let deps: Deps = graph.map(HashMap::from_iter).collect();
        let mut look = HashMap::new();
        let mut iter = deps.iter().enumerate();
        resolve(first.iter().map(|(a, b)| (a, b)), &iter, &mut look);
        let first: Vec<usize> = first.into_iter().map(|(b, _)| b).collect();
        while let Some((_, this_layer)) = iter.next() {
            resolve(this_layer.iter(), &iter, &mut look);
        }
        let first = Self::new_partition(first);
        Self { first, deps, look }
    }
}

impl TimeGraph {
    fn new(first: Partition<Vec<usize>>, deps: Deps, look: Look) -> Self {
        Self { first, deps, look }
    }

    fn new_partition(a: Vec<usize>) -> Partition<Vec<usize>> {
        let first_len = a.len();
        let mut first = Partition::new(a, first_len);
        first.next(); // comments this when switching the order
        first
    }

    fn step(&mut self) -> Option<(Self, Vec<usize>)> {
        // switch comments here to switch order
        let (measuring, mut first) = self.first.next()?;
        // let (mut first, measuring) = self.first.next()?;
        // if measuring.is_empty() {
        //     return None;
        // }
        let mut deps = self.deps.clone();
        let mut look = self.look.clone();
        for known in measuring.iter() {
            let dependents = look.remove(known).unwrap();
            for (layer, bit) in dependents {
                let dependencies = deps[layer].get_mut(&bit).unwrap();
                let pos = dependencies.iter().position(|e| e == known).unwrap();
                dependencies.swap_remove(pos);
                if dependencies.is_empty() {
                    deps[layer].remove(&bit).unwrap();
                    first.push(bit);
                }
            }
        }
        Some((Self::new(Self::new_partition(first), deps, look), measuring))
    }

    fn sweep(self) {
        let mut stack = Vec::new();
        let mut this = self;
        let mut path = Vec::new();
        loop {
            match this.step() {
                Some((that, mess)) => {
                    path.push(mess);
                    stack.push(mem::replace(&mut this, that));
                }
                None => {
                    this = match stack.pop() {
                        Some(old) => {
                            if this.first.set.is_empty() {
                                println!("{}; {:?}", path.len(), path);
                            }
                            path.pop();
                            old
                        }
                        None => {
                            break;
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use coverage_helper::test;

    use super::*;

    #[test]
    fn scheduler() {
        let graph = vec![
            vec![(0, vec![]), (2, vec![])],
            vec![(3, vec![0]), (1, vec![0, 2])],
            vec![(4, vec![0, 3])],
        ];
        println!("{:?}\n", graph);
        let mut scheduler = TimeGraph::from(graph);
        scheduler.sweep();
    }

    #[test]
    fn invert_graph() {
        let graph = vec![
            vec![(0, vec![]), (2, vec![])],
            vec![(3, vec![0]), (1, vec![0, 2])],
            vec![(4, vec![0, 3])],
        ];
        // println!("{:?}", graph);
        let graph = TimeGraph::from(graph);
        println!("{:?}\n{:?}\n{:?}", graph.first, graph.deps, graph.look);
    }
}
