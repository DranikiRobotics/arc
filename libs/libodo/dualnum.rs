#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct DualNum<T>(alloc::vec::Vec<T>);
