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
 * Defines the different states of the command.
 */
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum CmdStatus {
    /** When the command has been cancelled. */
    Cancelled,
    /** When the command is created but not executed yet. */
    Created,
    /** When the command has been marked as done. */
    Done,
    /** When the command has been created and executed one time. */
    Executed,
    /** The command has been flushed. In this case, the command must not be used anymore. */
    Flushed,
}

/**
 * A command is produced and executed in reaction of a user interaction.
 * It follows the command design pattern.
 * It contains statements to execute to perform the command.
 * The interface Undoable can be used to add undo/redo features to a command.
 */
pub struct Command<T: CustomCmd> {
    status: CmdStatus,
    pub child: T
}

impl<T: CustomCmd> Command<T> {
    pub fn new(child_cmd: T) -> Self {
        Self {
            status: CmdStatus::Created,
            child: child_cmd
        }
    }
}

impl<T: CustomCmd> Command<T> {
    pub fn get_status(&self) -> CmdStatus {
        self.status
    }

    pub fn set_status(&mut self, new_status: CmdStatus) {
        self.status = new_status
    }

    pub fn execute(&mut self) -> bool {
        let ok: bool;
        let status = self.get_status();
        if (status == CmdStatus::Created || status == CmdStatus::Executed) && self.child.can_execute() {
            if status == CmdStatus::Created {
                self.child.create_memento();
            }
            ok = true;

            //     try {
            self.execution();
            //         if (result instanceof Promise) {
            //             return result
            //                 .then(() => {
            //                     this.status = "executed";
            //                     return true;
            //                 })
            //                 .catch(() => {
            //                     this.status = "executed";
            //                     return false;
            //                 });
            //         }
            //     } catch (error: unknown) {
            //         this.status = "executed";
            //         throw error;
            //     }
            self.set_status(CmdStatus::Executed);
        } else {
            ok = false;
        }
        ok
    }

    /**
     * Actions may need to create a memento before their first execution.
     * This is the goal of the operation that should be overridden.
     * This operator is called a single time before the first execution of the command.
     */
    pub fn create_memento(&mut self) {
        self.child.create_memento()
    }

    pub fn can_execute(&self) -> bool {
        self.child.can_execute()
    }

    /**
     * This method contains the statements to execute the command.
     * This method is automatically called by 'execute' and must not be called explicitly.
     */
    pub fn execution(&mut self) {
        self.child.execution()
    }

    /**
     * Marks the command as "done" and sends it to the command registry.
     */
    pub fn done(&mut self) {
        if self.get_status() == CmdStatus::Created || self.get_status() == CmdStatus::Executed {
            self.set_status(CmdStatus::Done);
        }
    }

    /**
     * Flushes the UI command.
     * The command must not be used after that.
     */
    pub fn flush(&mut self) {
        self.set_status(CmdStatus::Flushed)
    }

    pub fn had_effect(&self) -> bool {
        self.is_done()
    }

    pub fn is_done(&self) -> bool {
        self.get_status() == CmdStatus::Done
    }

    pub fn cancel(&mut self) {
        self.set_status(CmdStatus::Cancelled)
    }
}

pub trait CustomCmd {
    /**
     * Actions may need to create a memento before their first execution.
     * This is the goal of the operation that should be overridden.
     * This operator is called a single time before the first execution of the command.
     */
    fn create_memento(&mut self) {
    }

    fn can_execute(&self) -> bool {
        true
    }

    /**
     * This method contains the statements to execute the command.
     * This method is automatically called by 'execute' and must not be called explicitly.
     */
    fn execution(&mut self);
}

pub trait UndoableCommand: CustomCmd + Undoable {}
