/*
 * Copyright (C) 2023 taylor.fish <contact@taylor.fish>
 *
 * This file is part of readline-sys.
 *
 * readline-sys is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * readline-sys is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with readline-sys. If not, see <https://www.gnu.org/licenses/>.
 *
 * readline-sys links to or is otherwise a derived work of GNU Readline.
 * See ./COPYRIGHT for more information.
 */

#![deny(unsafe_op_in_unsafe_fn)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

//! This crate contains low-level bindings to GNU Readline.
//!
//! This crate does not build Readline; it must already be installed on your
//! system.
//!
//! For API details, see [the documentation for GNU Readline][docs].
//!
//! [docs]: https://tiswww.cwru.edu/php/chet/readline/readline.html

use libc::{c_ulong, time_t, FILE};
use std::marker::{PhantomData, PhantomPinned};

mod bindgen;
pub use bindgen::*;

#[repr(C)]
pub struct KEYMAP_ENTRY {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[repr(C)]
pub struct readline_state {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn rl_clear_timeout() {
    unsafe {
        rl_set_timeout(0, 0);
    }
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn RL_SETSTATE(x: c_ulong) {
    unsafe {
        rl_readline_state |= x;
    }
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn RL_UNSETSTATE(x: c_ulong) {
    unsafe {
        rl_readline_state &= !x;
    }
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn RL_ISSTATE(x: c_ulong) -> c_ulong {
    (unsafe { rl_readline_state }) & x
}

#[repr(C)]
pub struct HIST_ENTRY {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[repr(C)]
pub struct HISTORY_STATE {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}
