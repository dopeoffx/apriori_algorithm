use std::collections::HashMap;
use std::hash::Hash;
use std::vec;
use std::cmp::{min, max};

use crate::dataset::Dataset;

type NodeId = usize;

#[derive(Debug)]
struct Node<T, K> 
where
    K: Eq + Hash,
{
    value: T,
    conter: i32,
    parent: Option<NodeId>,
    children: HashMap<K, NodeId>
}

#[derive(Debug)]
struct Apriori_Hash_Tree<T, K>
where 
    K: Eq + Hash + Clone,
{
    nodes: Vec<Option<Node<T, K>>>,
    free_nodes: Vec<NodeId>,
    root: NodeId,
    last_layer: Vec<NodeId>,
    input: Dataset
}
