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

fn main() {
    println!("cargo:rustc-link-lib=readline");
    println!("cargo:rerun-if-changed=build.rs");
}
