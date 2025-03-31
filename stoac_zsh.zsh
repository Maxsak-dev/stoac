stoac_widget() {
  local user_input="$BUFFER"
  local output

  # resetting the prompt to not show it before the program output
  LBUFFER=""
  RBUFFER=""
  zle reset-prompt

  output=$(stoac -l "$user_input" -p)

  # Check the exit status of the command
  if [[ $? -ne 0 ]]; then
    # print program output if fail
    printf "%s\n" "$output"

    # reset the buffers to not get stuck
    LBUFFER=""
    RBUFFER=""
    zle reset-prompt
  else
    # program succeeded -> use its output
    LBUFFER="$output"
    RBUFFER=""
    zle reset-prompt
  fi
}

zle -N stoac_widget

# You can customize the keybinding here (Standard ^H (Ctrl+H))
bindkey '^H' stoac_widget
