#[cfg(test)]
mod tests;

pub trait Environment<K, V> {
    fn define(&mut self, name: K, value: V) -> Option<V>;
    fn get(&mut self, name: &K) -> Option<&V>;
}

pub mod hashmap;
