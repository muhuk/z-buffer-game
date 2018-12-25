pub trait Menu<'a> {
    type Item: 'a + Clone + PartialEq;
    type IterMenu: Iterator<Item = &'a Self::Item> + Sized;

    fn iter(&self) -> Self::IterMenu;

    fn select_next(&mut self);
    fn select_previous(&mut self);

    fn is_selected(&self, item: &Self::Item) -> bool;
    fn selected(&self) -> Self::Item;
}
