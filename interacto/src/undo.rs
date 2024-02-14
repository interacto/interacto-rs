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

use crate::{undohistory::UndoHistoryBase, command::CustomCmd};

pub struct Undo {
    history: Box<dyn UndoHistoryBase>
}

impl Undo {
    pub fn new(history: Box<dyn UndoHistoryBase>) -> Self {
        Self {
            history
        }
    }
}

impl CustomCmd for Undo {
    fn execution(&mut self) {
        self.history.undo();
    }

    fn can_execute(&self) -> bool {
        self.history.get_last_undo().is_some()
    }
}

pub struct Redo {
    history: Box<dyn UndoHistoryBase>
}

impl Redo {
    pub fn new(history: Box<dyn UndoHistoryBase>) -> Self {
        Self {
            history
        }
    }
}

impl CustomCmd for Redo {
    fn execution(&mut self) {
        self.history.redo();
    }

    fn can_execute(&self) -> bool {
        self.history.get_last_redo().is_some()
    }
}

