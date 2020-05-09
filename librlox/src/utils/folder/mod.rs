// Folder defines a a fold method for translating a structure of one type to another.
pub trait Folder<T> {
    fn fold(&self) -> T;
}
