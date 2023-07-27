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

/**
 * An interface for undoable objects.
 */
pub trait Undoable {
    /**
     * Cancels the command.
     */
    fn undo(&mut self);

    /**
     * Redoes the cancelled command.
     */
    fn redo(&mut self);

    /**
     * @returns The name of the undo command.
     */
    fn get_undo_name(&self) -> String;

    // /**
    //  * Gives some information about the impact of a command.
    //  * @returns Information about the impact of the commmand as an SVG element or text.
    //  */
    // getVisualSnapshot() -> UndoableSnapshot;
}
