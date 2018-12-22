use std::slice::Iter;

pub trait Menu<'a> {
    type Item: Clone + PartialEq;

    fn iter(&self) -> Iter<'a, &'a Self::Item>;

    fn select_next(&mut self);
    fn select_previous(&mut self);

    fn is_selected(&self, item: &Self::Item) -> bool;
    fn selected(&self) -> Self::Item;
}
