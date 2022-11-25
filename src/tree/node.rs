use super::relation::Relation;
use std::collections::HashMap;
use std::vec::Vec;

pub struct Node {
    pub relation: Vec<Relation>,
    pub node_url: String,
    pub members: HashMap<String, String>,
}
