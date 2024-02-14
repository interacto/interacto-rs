/*
 * This file is part of Interacto.
 * Interacto is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * Interacto is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * You should have received a copy of the GNU General Public License
 * along with Interacto.  If not, see <https://www.gnu.org/licenses/>.
 */

use crate::{command::CustomCmd, undo::Undo, undoble::Undoable, undohistory::UndoHistoryBase};
use mockall::mock;

mock! {
    FakeHistory {}
    impl UndoHistoryBase for FakeHistory {
        fn undo(&self);
        fn redo(&self);
        fn clear(&self);
        fn add(&self, undoable: Box<dyn Undoable>);
        fn get_last_undo(&self) -> Option<&'static dyn Undoable>;
        fn get_last_redo(&self) -> Option<Box<dyn Undoable + 'static>>;
    }
}
mock! {
    FakeUndoable {}
    impl Undoable for FakeUndoable {
        fn undo(&mut self);
        fn redo(&mut self);
        fn get_undo_name(&self) -> String;
    }
}

#[test]
fn cannot_do() {
    let mut collector = MockFakeHistory::new();
    collector.expect_get_last_undo().returning(|| None);
    assert_eq!(
        Undo::new(Box::new(collector)).as_command().can_execute(),
        false
    );
}

// #[test]
// fn with_undoable() {
//     let mut collector = MockFakeHistory::new();
//     let undoable: MockFakeUndoable = MockFakeUndoable::new();
//     collector.expect_get_last_undo().returning(|| Some(&undoable));
//     assert_eq!(Undo::new(Box::new(collector)).as_command().can_execute(), true);
// }
