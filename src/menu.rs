// © Copyright 2019-2020, Atamert Ölçgen
//
// This file is part of z-buffer-game.
//
// z-buffer-game is free software: you can redistribute it and/or modify it
// under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or (at your
// option) any later version.
//
// z-buffer-game is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
// or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public
// License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with z-buffer-game.  If not, see <https://www.gnu.org/licenses/>.

pub trait Menu<'a> {
    type Item: 'a + Clone + PartialEq;
    type IterMenu: Iterator<Item = &'a Self::Item> + Sized;

    fn iter(&self) -> Self::IterMenu;

    fn select_next(&mut self);
    fn select_previous(&mut self);

    fn is_selected(&self, item: &Self::Item) -> bool;
    fn selected(&self) -> Self::Item;
}
