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
    printf "\e[1m\e[33m%s\e[0m" "$1"
}

function reset_terminal() {
    printf "\e[0m"
    stty echo
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
    local panel_width=$((COLUMNS - 20)) # Adjust the width to leave space for the borders and padding
    local panel_height=$((LINES - 2))   # Adjust the height to leave space for the top and bottom borders

    # Function to repeat a character
    repeat_char() {
        local char="$1"
        local times=$2
        printf "%0.s$char" $(seq 1 "$times")
    }

    print_vertical_line() {
        local char="$1"
        local padding=$((panel_width+1))
        printf "\e[19C%s%*s%s\n" "$char" "$padding" "$char"
    }

    printf "╭%s╮\n" "$(repeat_char "─" "$((panel_width - 2))")"
    title="GIT AUTOMATE"
    title_len=${#title}
    printf "\e[19C%*s%*s\n" "$((panel_width / 2 + title_len / 2))" "$title" "$((panel_width / 2 - title_len / 2))" ""

    local lines_to_print=$((panel_height - 3)) # Calculate how many vertical lines to print (excluding top and bottom borders and title lines)
    for ((i = 0; i < lines_to_print; i++)); do
        print_vertical_line "│"
    done

    printf "\e[19C╰%s╯\n" "$(repeat_char "─" "$((panel_width - 2))")"
}

function handle_resize() {
    set_terminal_dimensions
    redraw_menu
}

function redraw_menu() {
    printf "\e[2J\e[?25l"
    printf "\e[0;0H"
    stty -echo

    local num_items=${#menu_list[@]}
    for ((i = 0; i < num_items; i++)); do
        if [[ $i -eq $selected ]]; then
            highlight_item "$(draw_menu "➤ ${menu_list[i]}")"
        else
            printf "\e[2m"
            draw_menu "${menu_list[i]}"
            printf "\e[0m"
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
