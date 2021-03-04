use std::collections::HashMap;

#[derive(Default, Clone)]
/// OrderedSet maintains a consistent order of items determined by the sequence
/// that elements were added to the set.
struct OrderedSet<T> {
    v: Vec<T>,
    s: std::collections::HashSet<T>,
}

impl<T> OrderedSet<T>
where
    T: Eq + std::hash::Hash + Clone + Copy,
{
    /// If an element doesn't currently exist in a set, it is appended to the
    /// end of the set and true is returned.
    fn insert(&mut self, value: T) -> bool {
        if self.s.insert(value.clone()) {
            self.v.push(value);
            true
        } else {
            false
        }
    }
}

impl<T> OrderedSet<T>
where
    T: Clone + Copy,
{
    fn iter(&self) -> std::slice::Iter<T> {
        self.v.iter()
    }
}

impl<T> std::fmt::Debug for OrderedSet<T>
where
    T: std::fmt::Debug + Clone + Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "OrderedSet{{{}}}",
            self.clone()
                .iter()
                .map(|t| format!("{:?}", t))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

type ScopeId = usize;

/// Stores environmental data that is indexed of the Id of a scope.
#[derive(Default, Debug, Clone)]
pub struct Environment<V> {
    /// Stores scope indexed variable maps.
    data: Vec<HashMap<String, V>>,
    /// a parent of a scope indexed on the array id.
    parents: Vec<Option<ScopeId>>,
    /// stores a hashset of all child nodes of a scope aligned on the scope id.
    children: Vec<OrderedSet<ScopeId>>,
}

impl<V> Environment<V>
where
    V: Default + Clone,
{
    /// instantiates a new environment with a single global scope. By default,
    /// this scope's Id is 0.
    pub fn new() -> Self {
        Self {
            data: vec![HashMap::new()],
            children: vec![OrderedSet::default()],
            parents: vec![None],
        }
    }

    /// Generates a new scope in place, linking the new scope to it's parent.
    /// This returns the new ScopeId.
    pub fn add_scope_mut(&mut self, parent: ScopeId) -> ScopeId {
        let scope_id = self.data.len();
        // instantiate scope
        self.data.push(HashMap::new());
        self.children.push(OrderedSet::default());
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
    fn parent(&self, scope: ScopeId) -> Option<ScopeId> {
        self.parents.get(scope).and_then(|&scope| scope)
    }

    /// Returns the child scope ids for a given scope, or an empty vector.
    fn children(&self, scope: ScopeId) -> Vec<ScopeId> {
        self.children
            .get(scope)
            .map_or(vec![], |scope| scope.iter().copied().collect())
    }

    /// Returns true if a key is defined in a given scope. False is returned
    /// if the scope is undefined or if the key doesn't exist in the scope.
    fn defined_in_scope(&self, scope: ScopeId, name: &str) -> bool {
        self.data
            .get(scope)
            .and_then(|data| {
                if data.contains_key(name) {
                    Some(true)
                } else {
                    None
                }
            })
            .is_some()
    }

    /// find_first walks up the tree to the root node, returning the first
    /// occurrence of a variable. If one is not found, None is returned.
    /// Otherwise the scope id of the first occurrence of the variable is returened.
    fn find_first(&self, scope: ScopeId, key: &str) -> Option<ScopeId> {
        if self.defined_in_scope(scope, key) {
            Some(scope)
        } else {
            let mut current_scope = scope;
            while let Some(parent_scope) = self.parent(current_scope) {
                if self.defined_in_scope(parent_scope, key) {
                    return Some(parent_scope);
                } else {
                    current_scope = parent_scope;
                }
            }

            None
        }
    }

    /// Mutably defines a new variable in place for a scope. If the value was
    /// previously defined, Some(previous value) is returned. Otherwise, None
    /// is returned.
    pub fn define_mut(&mut self, scope: ScopeId, key: &str, value: V) -> Option<V>
    where
        V: Clone,
    {
        self.data
            .get_mut(scope)
            .and_then(|scope| scope.insert(key.to_string(), value))
    }

    /// Wraps define_mut, and defines a new variable for a scope. If the value
    /// was previously defined, Some(previous value) is returned with the
    /// modified environment otherwise the unomidified env and None is
    /// returned.
    pub fn define(mut self, scope: ScopeId, name: &str, value: V) -> (Self, Option<V>)
    where
        V: Clone,
    {
        let result = self.define_mut(scope, name, value);
        (self, result)
    }

    /// Assigns a new value to a variable already defined in scope. If found,
    /// Some(previous value) is returned. If the variable is undefined, none
    /// is returned.
    pub fn assign_mut(&mut self, scope: ScopeId, key: &str, value: V) -> Option<V> {
        if let Some(id) = self.find_first(scope, key) {
            self.define_mut(id, key, value)
        } else {
            None
        }
    }

    /// Wraps assign_mut and assigns a new value to a variable already defined
    /// in scope. if the variable is found Some(previous value) is returned
    /// with the modified environment. Otherwise, the unmodified environment and
    /// None is returned.
    pub fn assign(mut self, scope: ScopeId, key: &str, value: V) -> (Self, Option<V>) {
        let result = self.assign_mut(scope, key, value);
        (self, result)
    }

    /// Get walks up the tree, looking for the first occurrence of a variable.
    pub fn get(&self, scope: ScopeId, key: &str) -> Option<V> {
        self.find_first(scope, key)
            .and_then(|id| self.data[id].get(key).map(|v| v.clone()))
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

    #[test]
    fn should_define_variable_in_a_scope() {
        let (env, value) = Environment::<usize>::new().define(0, "test", 5);

        assert_eq!(None, value);
        assert_eq!(Some(&5), env.data[0].get("test"));
        assert!(env.defined_in_scope(0, "test"));
        assert!(!env.defined_in_scope(0, "not defined"));
    }

    #[test]
    fn should_get_variable_in_scope() {
        let (env, _) = Environment::<usize>::new().define(0, "test", 5);

        assert_eq!(Some(5), env.get(0, "test"));
    }

    #[test]
    fn should_get_variable_in_parent_scope() {
        let (env, child_scope) = Environment::<usize>::new().add_scope(0);
        let env = env.define(0, "test", 5).0;

        assert_eq!(Some(5), env.get(child_scope, "test"));
    }

    #[test]
    fn should_not_find_variable_in_adjacent_child_scope() {
        let (env, child_scope) = Environment::<usize>::new().add_scope(0);
        // assigns a value to a child scope off the root then forks an adjacent scope off the root
        let (env, neighbor_scope) = env.define(child_scope, "test", 5).0.add_scope(0);

        assert_eq!(Some(5), env.get(child_scope, "test"));
        assert_eq!(None, env.get(neighbor_scope, "test"));
    }

    #[test]
    fn should_assign_variable_in_a_scope() {
        let (env, child_scope) = Environment::<usize>::new()
            .define(0, "test", 5)
            .0
            .add_scope(0);

        let (env, previous_value) = env.assign(child_scope, "test", 10);

        assert_eq!(Some(5), previous_value);
        assert_eq!(Some(&10), env.data[0].get("test"));
    }
}
