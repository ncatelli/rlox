use std::collections::hash_set::HashSet;
use std::collections::HashMap;

type ScopeId = usize;

/// Stores environmental data that is indexed of the Id of a scope.
#[derive(Default, Debug, Clone)]
pub struct Environment<V> {
    /// Stores scope indexed variable maps.
    data: Vec<HashMap<String, V>>,
    /// a parent of a scope indexed on the array id.
    parents: Vec<Option<ScopeId>>,
    /// stores a hashset of all child nodes of a scope aligned on the scope id.
    children: Vec<HashSet<ScopeId>>,
}

impl<V> Environment<V>
where
    V: Default,
{
    /// instantiates a new environment with a single global scope. By default,
    /// this scope's Id is 0.
    pub fn new() -> Self {
        Self {
            data: vec![HashMap::new()],
            children: vec![HashSet::new()],
            parents: vec![None],
        }
    }

    /// Generates a new scope in place, linking the new scope to it's parent.
    /// This returns the new ScopeId.
    pub fn add_scope_mut(&mut self, parent: ScopeId) -> ScopeId {
        let scope_id = self.data.len();
        // instantiate scope
        self.data.push(HashMap::new());
        self.children.push(HashSet::new());
        self.parents.push(Some(parent));

        // add child link to parent.
        self.children[parent].insert(scope_id);

        // return the index into the data vector representing the new scope.
        scope_id
    }

    /// Add scope wraps add_scope_mut, returning both the environment and new scope id.
    pub fn add_scope(mut self, parent: ScopeId) -> (Self, ScopeId) {
        let scope_id = self.add_scope_mut(parent);
        (self, scope_id)
    }

    /// Returns the parent scope id for a given scope, or None if the scope is
    /// global or undefined.
    pub fn parent(&self, scope: ScopeId) -> Option<ScopeId> {
        self.parents.get(scope).and_then(|&scope| scope)
    }

    /// Returns the child scope ids for a given scope, or an empty vector.
    pub fn children(&self, scope: ScopeId) -> Vec<ScopeId> {
        self.children
            .get(scope)
            .map_or(vec![], |scope| scope.iter().copied().collect())
    }
}

mod tests {
    use super::*;

    #[test]
    fn new_scopes_should_be_linked_to_their_parent() {
        let parent_scope_id = 0;
        let (env, child_scope_id) = Environment::<usize>::new().add_scope(0);
        assert_eq!(None, env.parent(parent_scope_id));
        assert_eq!(Some(parent_scope_id), env.parent(child_scope_id));
        assert_eq!(vec![1], env.children(parent_scope_id));
    }
}
