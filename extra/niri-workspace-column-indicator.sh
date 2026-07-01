#!/usr/bin/env bash

# A Niri workspace-column-indicator module for Waybar. Displays which column in the workspace has focus.
#
# Dependencies: jq
#
# https://github.com/willemw12/niri-workspace-column-indicator, GPL-3.0-or-later

main() {
  print_columns_info

  niri msg event-stream | while read -r line; do
    # printf 'DEBUG: line=%s\n' "$line" >&2
    case "$line" in
      # 'Window opened or changed': when a column could be created
      # 'Window closed': when a column on another workspace could be deleted
      # 'Window layouts changed': when a window consume/expel could delete/create a column
      'Window focus changed: '* | 'Window opened or changed: '* | 'Window closed: '* | 'Window layouts changed: '*)
        print_columns_info
        ;;
    esac
  done
}

print_columns_info() {
  local focused_window_json
  focused_window_json=$(niri msg -j focused-window)
  if [[ -n "$focused_window_json" && "$focused_window_json" != null ]]; then
    local workspace_id column_pos
    workspace_id=$(echo -n "$focused_window_json" | jq -r '.workspace_id')
    column_pos=$(echo -n "$focused_window_json" | jq -r '.layout.pos_in_scrolling_layout[0]')
  fi

  local total_columns
  total_columns=$(niri msg -j windows | jq -r "[.[] | select(.workspace_id == $workspace_id and .is_floating == false) | .layout.pos_in_scrolling_layout[0]] | max // empty")

  ####

  local bar column_text=
  bar=$(for ((i = 1; i <= total_columns; i++)); do ((i == column_pos)) && echo -n ' ' || echo -n ' '; done) # ' '
  [[ -n "$column_pos" && "$column_pos" != null ]] && column_text="  ($column_pos)"
  printf '%s%s\n' "$bar" "$column_text"
}

main "$@"
