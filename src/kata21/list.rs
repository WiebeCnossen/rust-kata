pub trait List<T: PartialEq> {
    type Node;

    fn empty() -> Self;

    fn add(&self, T) -> Self;

    fn find(&self, &T) -> Option<&Self::Node>;

    fn remove(&self, &Self::Node) -> Self;

    fn collect(&self) -> Vec<&T>;
}
