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

use crate::undoble::Undoable;

/**
 * The base trait for undo histories.
 */
pub trait UndoHistoryBase {
    /**
     * Undoes the last undoable object.
     */
    fn undo(&self);

    /**
     * Redoes the last undoable object.
     */
    fn redo(&self);

    /**
     * Removes all the undoable objects of the collector.
     */
    fn clear(&self);

    /**
     * Adds an undoable object to the collector.
     * @param undoable - The undoable object to add.
     */
    fn add(&self, undoable: Box<dyn Undoable>);

    /**
     * @returns The last undoable object or undefined if there is no last object.
     */
    fn get_last_undo(&self) -> Option<&dyn Undoable>;

    /**
     * @returns The last redoable object or undefined if there is no last object.
     */
    fn get_last_redo(&self) -> Option<Box<dyn Undoable>>;

    // /**
    //  * @returns The last undoable object name or undefined if there is no last object.
    //  */
    // fn getLastUndoMessage(): string | undefined;

    // /**
    //  * @returns The last redoable object name or undefined if there is no last object.
    //  */
    // fn getLastRedoMessage(): string | undefined;

    // /**
    //  * @returns The last undoable object name or an empty string if there is no last object.
    //  */
    // fn getLastOrEmptyUndoMessage(): string;

    // /**
    //  * @returns The last redoable object name or an empty string if there is no last object.
    //  */
    // fn getLastOrEmptyRedoMessage(): string;

    // /**
    //  * A stream for observing changes regarding the last undoable object.
    //  * @returns An observable value of optional undoable objects: if empty, this means
    //  * that no undoable object are stored anymore.
    //  */
    // undosObservable(): Observable<Undoable | undefined>;

    // /**
    //  * A stream for observing changes regarding the last redoable object.
    //  * @returns An observable value of optional redoable objects: if empty, this means
    //  * that no redoable object are stored anymore.
    //  */
    // redosObservable(): Observable<Undoable | undefined>;
}

pub trait UndoHistory: UndoHistoryBase {
    /**
     * @returns The stack of saved undoable objects.
     */
    fn get_undo(&self) -> Vec<Box<dyn Undoable>>;

    /**
     * @returns The stack of saved redoable objects
     */
    fn get_redo(&self) -> Vec<Box<dyn Undoable>>;

    /**
     * @returns The max number of saved undoable objects.
     */
    fn get_size_max(&self) -> u64;

    /**
     * @param max - The max number of saved undoable objects. Must be great than 0.
     */
    fn set_size_max(&mut self, max: u64);
}
