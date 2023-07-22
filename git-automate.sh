#!/bin/bash

RESET="\e[0m"
BLACK="\e[30m"
RED="\e[31m"
GREEN="\e[32m"
YELLOW="\e[33m"
BLUE="\e[34m"
MAGENTA="\e[35m"
CYAN="\e[36m"
WHITE="\e[37m"

BRIGHT_BLACK="\e[90m"
BRIGHT_RED="\e[91m"
BRIGHT_GREEN="\e[92m"
BRIGHT_YELLOW="\e[93m"
BRIGHT_BLUE="\e[94m"
BRIGHT_MAGENTA="\e[95m"
BRIGHT_CYAN="\e[96m"
BRIGHT_WHITE="\e[97m"

BG_BLACK="\e[40m"
BG_RED="\e[41m"
BG_GREEN="\e[42m"
BG_YELLOW="\e[43m"
BG_BLUE="\e[44m"
BG_MAGENTA="\e[45m"
BG_CYAN="\e[46m"
BG_WHITE="\e[47m"

BG_BRIGHT_BLACK="\e[100m"
BG_BRIGHT_RED="\e[101m"
BG_BRIGHT_GREEN="\e[102m"
BG_BRIGHT_YELLOW="\e[103m"
BG_BRIGHT_BLUE="\e[104m"
BG_BRIGHT_MAGENTA="\e[105m"
BG_BRIGHT_CYAN="\e[106m"
BG_BRIGHT_WHITE="\e[107m"

CURSOR_UP="\e[A"
CURSOR_DOWN="\e[B"
CURSOR_FORWARD="\e[C"
CURSOR_BACK="\e[D"
CURSOR_NEXT_LINE="\e[E"
CURSOR_PREV_LINE="\e[F"
CURSOR_COLUMN="\e[G"
CURSOR_POSITION="\e[{row};{col}H"
CURSOR_SAVE="\e[s"
CURSOR_RESTORE="\e[u"
CURSOR_HIDE="\e[?25l"
CURSOR_SHOW="\e[?25h"

BOLD="\e[1m"
DIM="\e[2m"
ITALIC="\e[3m"
UNDERLINE="\e[4m"
BLINK="\e[5m"
FAST_BLINK="\e[6m"
REVERSE="\e[7m"
HIDDEN="\e[8m"
CROSSED="\e[9m"
FRAMED="\e[51m"
ENCIRCLED="\e[52m"
OVERLINED="\e[53m"

CLEAR_SCREEN="\e[2J"
CLEAR_LINE="\e[2K"
CLEAR_LINE_FROM_CURSOR="\e[1K"
CLEAR_LINE_TO_CURSOR="\e[0K"
ENABLE_ALTERNATE_SCREEN="\e[?1049h"
DISABLE_ALTERNATE_SCREEN="\e[?1049l"
SET_TITLE="\e]0;{title}\a"
BELL="\a"
CARRIAGE_RETURN="\r"
NEWLINE="\n"
TAB="\t"

menu_list=(
    "Status"
    "Staging"
    "Semantic"
    "Simple"
    "Edit"
    "Delete"
    "Automate"
    "Stash"
    "Branches"
    "Tag"
    "Reset"
    "Revert"
    "Quit"
)

PANELS=("background", "title", "interactive")
PANELS_LEN=${#PANELS[@]}

get_terminal_size() {
    read -r ROW COL < <(stty size)
    PANEL_HEIGHT=$((ROW - (2 * MARGIN)))
    PANEL_WIDTH=$((COL - (2 * MARGIN)))
    INDENT=$((PANEL_WIDTH / 100))
    MARGIN=$((PANEL_WIDTH / 80))
}

move_cursor() {
    printf "\e[%d;%dH" "$(($1 + 1))" "$(($2 + 1))" # +1 compensates the 0-based index
}

display_message() {
    local message="$1"
    local message_row=$((ROW / 2))
    local message_col=$((COL / 2 - (${#message} / 2)))
    move_cursor "$message_row" "$message_col"
    echo "$message"
}

draw_panel() {
    local title="$1"
    local tabulation="${2:-0}"
    local padding="$((${3:-0} - ${#title} < 0 ? 0 : ${3:-0} - ${#title}))"
    local panels_num=$((${4:-0} < 0 ? 0 : ${4:-0}))

    local panel_indent=2
    local horizontal_line="─$title─"
    if ((panels_num > 0)); then
        printf "${panel_indent}╭${horizontal_line//?/─}╮"
        for ((i = 0; i < panels_num; i++)); do
            printf "│ ${title} │"
        done
        printf "╰${horizontal_line//?/─}╯" # ends with newline
        ((panel_indent += 2))
    fi

    # Simple bordered text
    local blank="          "
    local horizontal_line="─$title${blank:0:padding}─"
    printf "╭%*s${horizontal_line//?/─}╮\n" $tabulation
    printf "│%*s ${title}${blank:0:padding} │\n" $tabulation
    printf "╰%*s${horizontal_line//?/─}╯\r" $tabulation # ends with carriage return
}

highlight_item() {
    printf "$BOLD$2%s$RESET" "$1"
}

render_menu() {
    local num_items=${#menu_list[@]}
    move_cursor 3 0
    draw_panel " COMMITS "
    for ((i = 0; i < num_items; i++)); do
        move_cursor $((6 + (i * 2))) ""
        if [[ $i -eq $SELECTED ]]; then
            if [[ $i -eq $((num_items - 1)) ]]; then
                highlight_item "$(draw_panel "➤ ${menu_list[i]}" "" 10)" "$RESET$RED"
                break
            fi
            highlight_item "$(draw_panel "➤ ${menu_list[i]}" "" 10)" "$YELLOW"
        else
            printf "$DIM"
            draw_panel "${menu_list[i]}" 0 10
            printf "$RESET"
        fi
    done
}

handle_resize() {
    get_terminal_size
    if ((COL >= 60 && ROW >= 20)); then
        draw_interface=true
    else
        draw_interface=false
        display_message "Terminal size is too small to display properly. Please resize the terminal window."
    fi
}

background() {
    local width=$((COLUMNS - 6))
    local height=$((LINES - 4))

    horizontal_line() {
        local char="$1"
        printf "%0.s$char" $(seq 1 "$width")
    }

    vertical_line() {
        local char="$1"
        printf "%s%*s%s\n" "$char" "$width" "" "$char"
    }

    printf "$DIM$BRIGHT_BLACK"
    printf "%*s╭%s╮\n" "$((INDENT - 1))" "" "$(horizontal_line "─")"

    for ((i = 0; i < height; i++)); do
        printf "%*s" "$((INDENT - 1))" ""
        vertical_line "│"
    done

    printf "%*s╰%s╯\n" "$((INDENT - 1))" "" "$(horizontal_line "─")"
    printf "$RESET"
}

update_interface() {
    background
    render_menu

    draw_interface=false
}

async_handle_key_press() {
    IFS= read -rsn1 -t 0.1 key

    case "$key" in
    "q") # 'q' key pressed, exit the script
        reset_terminal
        exit
        ;;
    "A") # Up arrow key pressed
        ((SELECTED = (SELECTED - 1 + num_items) % num_items))
        draw_interface=true
        ;;
    "B") # Down arrow key pressed
        ((SELECTED = (SELECTED + 1) % num_items))
        draw_interface=true
        ;;
    "C") # Right arrow key pressed
        ((SELECTED = (SELECTED + 1) % num_items))
        draw_interface=true
        ;;
    "D") # Left arrow key pressed
        ((SELECTED = (SELECTED - 1 + num_items) % num_items))
        draw_interface=true
        ;;
    $'\0') # Enter key pressed
        printf "\n\e[2;20H SELECTED: %s\n" "${menu_list[SELECTED]}"
        ;;
    esac
}

reset_terminal() {
    printf "$CURSOR_SHOW"
    stty sane
}

main() {
    local menu=("$@")
    local num_items=${#menu[@]}

    SELECTED=0
    draw_interface=true

    # Register the resize signal handler
    trap handle_resize SIGWINCH

    while true; do
        if [ "$draw_interface" = true ]; then
            clear
            printf "$CURSOR_HIDE"
            stty -echo
            update_interface
        fi

        # Set the terminal in non-blocking mode for reading input
        stty -icanon time 0 min 0

        # Wait for user input with a timeout of 0.1 seconds (auto-select option)
        # This allows the script to choose the option while unblock resize signals
        async_handle_key_press

        # Reset the terminal to blocking mode for the trap to work properly
        stty sane
    done
}

get_terminal_size
handle_resize
main "${menu_list[@]}"
