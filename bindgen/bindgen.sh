#!/bin/sh
# Copyright (C) 2023 taylor.fish <contact@taylor.fish>
#
# This file is part of gnu-readline-sys.
#
# gnu-readline-sys is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# gnu-readline-sys is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with gnu-readline-sys. If not, see <https://www.gnu.org/licenses/>.
#
# gnu-readline-sys links to or is otherwise a derived work of GNU Readline.
# See ./COPYRIGHT for more information.

set -eu
case "${1:--h}" in
    -h|--help)
        echo >&2 "Usage: $(basename "$0") <readline-dir>"
        exit 1
    ;;
esac

echo "//! Generated from $(basename "$1")."
echo "use super::*;"
echo
bindgen "$(dirname "$0")/bindgen.h" \
    --allowlist-file "$(realpath "$1/readline.h")" \
    --allowlist-file "$(realpath "$1/history.h")" \
    --blocklist-type FILE \
    --blocklist-type KEYMAP_ENTRY \
    --blocklist-type readline_state \
    --blocklist-type _funmap \
    --blocklist-type FUNMAP \
    --blocklist-item funmap \
    --blocklist-type undo_list \
    --blocklist-type UNDO_LIST \
    --blocklist-item rl_undo_list \
    --blocklist-type _keymap_entry \
    --blocklist-type __time_t \
    --blocklist-type time_t \
    --blocklist-type _hist_entry \
    --blocklist-type HIST_ENTRY \
    --blocklist-type _hist_state \
    --blocklist-type HISTORY_STATE \
    -- \
    -iquote "$1"
