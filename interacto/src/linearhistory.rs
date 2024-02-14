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

use crate::{undoble::Undoable, undohistory::{UndoHistory, UndoHistoryBase}};

pub struct LinearHistoryImpl {
    /**
     * Contains the undoable objects.
     */
    undos: Vec<Box<dyn Undoable>>,

    /**
     * Contains the redoable objects.
     */
    redos: Vec<Box<dyn Undoable>>,

    /**
     * The maximal number of undo.
     */
    size_max: u64,
    // private readonly undoPublisher: Subject<Undoable | undefined>;

    // private readonly redoPublisher: Subject<Undoable | undefined>;
}

impl LinearHistoryImpl {
    pub fn new() -> Self {
        Self {
            size_max: 100,
            undos: Vec::new(),
            redos: Vec::new(),
        }
    }
}

impl UndoHistoryBase for LinearHistoryImpl {
    fn undo(&self) {
        todo!()
    }

    fn redo(&self) {
        todo!()
    }

    fn clear(&self) {
        todo!()
    }

    fn add(&self, undoable: Box<dyn Undoable>) {
        todo!()
    }

    fn get_last_undo(&self) -> Option<&dyn Undoable> {
        todo!()
    }

    fn get_last_redo(&self) -> Option<Box<dyn Undoable>> {
        todo!()
    }
}

impl UndoHistory for LinearHistoryImpl {
    fn get_undo(&self) -> Vec<Box<dyn Undoable>> {
        todo!()
    }

    fn get_redo(&self) -> Vec<Box<dyn Undoable>> {
        todo!()
    }

    fn get_size_max(&self) -> u64 {
        todo!()
    }

    fn set_size_max(&mut self, max: u64) {
        todo!()
    }
}
