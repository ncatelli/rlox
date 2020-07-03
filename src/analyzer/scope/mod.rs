use crate::analyzer::SemanticAnalyzer;
use crate::ast::statement::Stmt;
use std::collections::{HashMap, VecDeque};
use std::fmt;

#[cfg(test)]
mod tests;

pub type Scope = HashMap<String, usize>;

#[derive(PartialEq, Debug)]
pub struct Node {
    data: Scope,
    children: Option<VecDeque<Node>>,
}

impl Node {
    pub fn new() -> Self {
        Node {
            data: Scope::new(),
            children: None,
        }
    }

    pub fn add_child(self, child: Node) -> Self {
        let mut node = self;
        let children = node.children;

        node.children = match children {
            None => Some(vec![child].into_iter().collect()),
            Some(v) => {
                let mut v = v;
                v.push_back(child);
                Some(v)
            }
        };

        node
    }
}

impl Into<Scope> for Node {
    fn into(self) -> Scope {
        self.data
    }
}

impl Into<Vec<Scope>> for Node {
    fn into(self) -> Vec<Scope> {
        vec![self.data]
            .into_iter()
            .chain(
                self.children
                    .unwrap_or(VecDeque::new())
                    .into_iter()
                    .map(|n| {
                        let node: Vec<Scope> = n.into();
                        node
                    })
                    .flatten()
                    .into_iter(),
            )
            .collect()
    }
}

#[derive(PartialEq, Debug)]
pub enum ScopeAnalyzerErr {
    Unspecified,
    Unimplemented, // Eventually remove after completed
}

impl fmt::Display for ScopeAnalyzerErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unspecified => write!(f, "unspecified resolver error"),
            Self::Unimplemented => write!(f, "This endpoint was not implemented yet"),
        }
    }
}

pub struct ScopeAnalyzer {}

impl ScopeAnalyzer {
    pub fn new() -> Self {
        ScopeAnalyzer {}
    }
}

impl SemanticAnalyzer<&Vec<Stmt>, Node> for ScopeAnalyzer {
    type Error = ScopeAnalyzerErr;

    fn analyze(&self, input: &Vec<Stmt>) -> Result<Node, Self::Error> {
        let node = Node::new();
        self.analyze((node, input))
    }
}

impl SemanticAnalyzer<(Node, &Vec<Stmt>), Node> for ScopeAnalyzer {
    type Error = ScopeAnalyzerErr;

    fn analyze(&self, input: (Node, &Vec<Stmt>)) -> Result<Node, Self::Error> {
        let (mut node, stmts) = input; // unpack input
        for stmt in stmts.into_iter() {
            node = self.analyze((node, stmt))?
        }

        Ok(node)
    }
}

impl SemanticAnalyzer<(Node, &Stmt), Node> for ScopeAnalyzer {
    type Error = ScopeAnalyzerErr;

    fn analyze(&self, input: (Node, &Stmt)) -> Result<Node, Self::Error> {
        let (node, stmt) = input; // unpack input
        let result_node = match stmt {
            Stmt::Block(stmts) => self.analyze_block(node, stmts),
            _ => Err(ScopeAnalyzerErr::Unimplemented),
        };
        result_node
    }
}

impl ScopeAnalyzer {
    fn analyze_block(&self, node: Node, stmts: &Vec<Stmt>) -> Result<Node, ScopeAnalyzerErr> {
        let child = self.analyze((Node::new(), stmts))?;
        Ok(node.add_child(child))
    }
}
