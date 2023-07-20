
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
    "Show Commits"
    "Staging Area"
    "Simple Commit"
    "Edit Commits"
    "Semantic Commit"
    "Delete Commits"
    "Stash Changes"
    "Apply Stash"
    "Show Branches"
    "Create Branch"
    "Checkout Branch"
    "Delete Branch"
    "Merge Branch"
    "Tag Commit"
    "Close Program"
)

function set_terminal_dimensions() {
    read -r COLUMNS LINES < <(stty size)
}

function highlight_item() {
    printf "$BOLD$YELLOW%s$RESET" "$1"
}

function reset_terminal() {
    printf "$RESET"
    printf "$CURSOR_SHOW"
}

function draw_menu() {
    local content="$1"
    local content_width="${#content}"
    local max_width=$((COLUMNS - 4)) # adjust for box borders

    if ((content_width > max_width)); then
        content="${content:0:max_width}"
        content_width="$max_width"
    fi

    local blank="          "
    local padding=$((17 - content_width))
    local horizontal_line="─$content${content:0:padding}─"

    printf "╭${horizontal_line//?/─}╮\n"
    printf "│ ${content} ${blank:0:padding}│\n"
    printf "╰${horizontal_line//?/─}╯\r"
}

function draw_panel() {
    local content="$1"
    local panel_width=$((COLUMNS - 20)) # adjust the width to leave space for the borders and padding
    local panel_height=$((LINES - 4))   # adjust the height to leave space for the top and bottom borders

    horizontal_line() {
        local char="$1"
        local times=$2
        if ((panel_width % 2 == 0)); then
            printf "%0.s$char" $(seq 1 "$times")
        else # accounting for odd numbers
            printf "%0.s$char" $(seq 1 "$((times + 1))")
        fi
    }

    vertical_line() {
        local char="$1"
        local padding=$((panel_width))
        printf "\e[19C%s%*s%s\n" "$char" "$padding" "$char"
    }

    printf "╭%s╮\n" "$(horizontal_line "─" "$((panel_width - 4))")"
    title="GIT AUTOMATE"
    title_len=0
    if ((${#title_len} % 2 == 0)); then
        title_len=${#title}
    else
        title_len=$((${#title} - 1))
    fi
    # TODO: fix border alignment
    local left_padding=$((panel_width / 2 + title_len / 2))
    local right_padding=$((panel_width / 2 - title_len / 2 - 1 - (panel_width % 2)))
    printf "\e[19C│%*s%*s│\n" "$left_padding" "$title" "$right_padding" ""

    local lines_to_print=$((panel_height - 3)) # calculate how many vertical lines to print (excluding top and bottom borders and title lines)
    for ((i = 0; i < lines_to_print; i++)); do
        vertical_line "│"
    done

    printf "\e[19C╰%s╯\n" "$(horizontal_line "─" "$((panel_width - 4))")"
}

function handle_resize() {
    set_terminal_dimensions
    redraw_menu
}

function redraw_menu() {
    printf "$CLEAR_SCREEN$CURSOR_HIDE"
    printf "\e[0;0H"
    stty -echo

    local num_items=${#menu_list[@]}
    for ((i = 0; i < num_items; i++)); do
        if [[ $i -eq $selected ]]; then
            highlight_item "$(draw_menu "➤ ${menu_list[i]}")"
        else
            printf "$DIM"
            draw_menu "${menu_list[i]}"
            printf "$RESET"
        fi
    done

    printf "\e[0;20H"
}

function display_menu() {
    local menu=("$@")
    local num_items=${#menu[@]}

    selected=0
    handle_resize SIGWINCH
    redraw_menu

    while true; do
        local key
        IFS= read -rsn1 key

        case "$key" in
        "q") # 'q' key pressed, exit the script
            reset_terminal
            tput cnorm
            break
            ;;
        "A") # Up arrow key pressed
            ((selected = (selected - 1 + num_items) % num_items))
            redraw_menu
            ;;
        "B") # Down arrow key pressed
            ((selected = (selected + 1) % num_items))
            redraw_menu
            ;;
        "C") # Righ arrow key pressed
            ((selected = (selected + 1) % num_items))
            redraw_menu
            ;;
        "D") # Left arrow key pressed
            ((selected = (selected - 1 + num_items) % num_items))
            redraw_menu
            ;;
        "") # Enter key pressed
            local panel_content="This is the panel content"
            draw_panel "$panel_content"

            printf "You selected: %s\n" "${menu[selected]}"
            ;;
        esac
    done

    exit 0
}

display_menu "${menu_list[@]}"
