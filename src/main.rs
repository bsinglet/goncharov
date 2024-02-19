#[macro_use]
extern crate crossterm;

use std::fmt;
use std::fs::File;
use std::io::{self, stdout, Write};
//use std::io::prelude::*;
use std::io::BufWriter;
use crossterm::cursor;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};

#[derive(Debug, Clone)]
enum Buffer {
    Add,
    Original,
}

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result: String = match self {
            Buffer::Add => "Add".to_string(),
            Buffer::Original => "Original".to_string(),
        };
        write!(f, "{}", result.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct PieceTable {
    which: Vec<Buffer>,
    start: Vec<usize>,
    end: Vec<usize>,
}

impl fmt::Display for PieceTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result: String = "".to_string();
        for each_index in 0..self.which.len() {
            result += &format!("{} | {} | {}\n", &self.which[each_index], &self.start[each_index], &self.end[each_index]).as_str();
        }
        write!(f, "{}", result.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct CursorState {
    desired_x: usize,
    x: usize,
    y: usize,
    clip_right: bool,
}

impl fmt::Display for CursorState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result: String = format!("desired_x: {}", &self.desired_x);
        result += format!("(x, y): ({},{})", &self.x, &self.y).as_str();
        result += format!("clip_right: {}", &self.clip_right).as_str();
        write!(f, "{}", result.as_str())
    }
}

fn _test_text() {
    let mut piece_table: PieceTable = PieceTable{
        which: Vec::new(),
        start: Vec::new(),
        end: Vec::new(),
    };
    let original_buffer: String;
    let mut add_buffer: String;
    piece_table.which.push(Buffer::Original);
    piece_table.start.push(0);
    piece_table.end.push(11);
    original_buffer = String::from("Hello world");
    add_buffer = String::from("You");
    println!("{}", read_table(&piece_table, &original_buffer, &add_buffer));
    piece_table.end[0] = 6;
    piece_table.which.push(Buffer::Add);
    piece_table.start.push(0);
    piece_table.end.push(3);
    println!("{}", read_table(&piece_table, &original_buffer, &add_buffer));
    let insertion_text: String = "to ".to_string();
    (add_buffer, piece_table) = insert_table(add_buffer, piece_table, &insertion_text, 6);
    println!("{}", read_table(&piece_table, &original_buffer, &add_buffer));
    println!("Original buffer: {}", original_buffer);
    println!("Add buffer: {}", add_buffer);
    println!("{}", piece_table);
}

fn _delete_text(mut piece_table: PieceTable, delete_start: usize, delete_end: usize) -> PieceTable {
    

    piece_table
}

fn insert_table(mut add_buffer: String, mut piece_table: PieceTable, insertion_text: &String, insert_index: usize) -> (String, PieceTable) {
    let mut found: bool = false;
    let mut entry_num: usize = 0;
    let mut current_index: usize = 0;
    for each_index in 0..piece_table.which.len() {
        if current_index == insert_index {
            entry_num = each_index;
            found = true;
            break;
        }
        if current_index + (piece_table.end[each_index] - piece_table.start[each_index]) > insert_index {
            entry_num = each_index;
            found = true;
            break;
        }
        current_index += piece_table.end[each_index] - piece_table.start[each_index];
    }

    // the new table entry will be after all the others
    if !found {
        piece_table.which.push(Buffer::Add);
        piece_table.start.push(add_buffer.len());
        piece_table.end.push(add_buffer.len() + insertion_text.len());
        add_buffer += insertion_text;
    } else {
        if current_index == insert_index {
            // prepend before entry_num
            piece_table.which.splice(entry_num..entry_num, vec![Buffer::Add]);
            piece_table.start.splice(entry_num..entry_num, vec![add_buffer.len()]);
            piece_table.end.splice(entry_num..entry_num, vec![add_buffer.len() + insertion_text.len()]);
        }  else {
            // split the text in entry_num
            let old_start: usize = piece_table.start[entry_num];
            let old_end: usize = piece_table.end[entry_num];
            piece_table.end[entry_num] = old_start + (insert_index - current_index);
            piece_table.which.splice(entry_num+1..entry_num+1, vec![Buffer::Add]);
            piece_table.start.splice(entry_num+1..entry_num+1, vec![add_buffer.len()]);
            piece_table.end.splice(entry_num+1..entry_num+1, vec![add_buffer.len() + insertion_text.len()]);
            piece_table.which.splice(entry_num+2..entry_num+2, vec![piece_table.which[entry_num].clone()]);
            piece_table.start.splice(entry_num+2..entry_num+2, vec![old_start + (insert_index - current_index)]);
            piece_table.end.splice(entry_num+2..entry_num+2, vec![old_end]);
        }
        add_buffer += insertion_text;
    }

    (add_buffer, piece_table)
}

fn read_table(piece_table: &PieceTable, original_buffer: &String, add_buffer: &String) -> String {
    let mut message: String = "".to_string();
    for each_index in 0..piece_table.which.len() {
        message += match piece_table.which[each_index] {
            Buffer::Add => &add_buffer[piece_table.start[each_index]..piece_table.end[each_index]],
            Buffer::Original => &original_buffer[piece_table.start[each_index]..piece_table.end[each_index]],
        };
    }
    message
}

fn _get_table_length(piece_table: &PieceTable) -> usize {
    let mut table_length: usize = 0;
    for each_index in 0..piece_table.which.len() {
        table_length += piece_table.end[each_index] - piece_table.start[each_index];
    }
    table_length
}

fn _cursor_left(move_by: i32) {
    print!("\x1B[{}D", move_by);
    io::stdout().flush().unwrap();
}

fn _cursor_right(move_by: i32) {
    print!("\x1B[{}C", move_by);
    io::stdout().flush().unwrap();
}

fn _cursor_up(move_by: i32) {
    print!("\x1B[{}A", move_by);
    io::stdout().flush().unwrap();
}

fn _cursor_down(move_by: i32) {
    print!("\x1B[{}B", move_by);
    io::stdout().flush().unwrap();
}

fn _make_text_red(text: &String) -> String {
    format!("\x1b[31m{}\x1b[0m", &text)
}

fn make_text_green(text: &String) -> String {
    format!("\x1b[32m{}\x1b[0m", &text)
}

fn _make_text_blue(text: &String) -> String {
    format!("\x1b[34m{}\x1b[0m", &text)
}

fn _clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn get_position_of_offset(message: &String, offset: usize, line_width: usize, line_wrap: bool) -> (usize, usize) {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut current_offset: usize = 0;
    for each_character in message.as_bytes() {
        // break on edge of line if line_wrap
        if line_wrap && x == line_width {
            y += 1;
            x = 0;
        }
        if current_offset == offset {
            break;
        }
        // newline is always a break, even if we just broke because of the end of a line.
        if each_character == &('\n' as u8) {
            y += 1;
            x = 0;
        } else {
            x += 1;
        }
        current_offset += 1;
    }
    // go to the next line if you move the cursor to the position after the last character
    if line_wrap && offset == message.len() {
        if x == line_width {
            x = 0;
            y += 1;
        }
    }
    (x, y)
}

fn get_offset_of_position(message: &String, pos_x: usize, pos_y: usize, line_width: usize, line_wrap: bool) -> usize {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut current_offset: usize = 0;
    for each_character in message.as_bytes() {
        if y == pos_y && x == pos_x {
            break;
        }
        // break on edge of line if line_wrap
        if line_wrap && x == line_width {
            y += 1;
            x = 0;
        }
        // newline is always a break, even if we just broke because of the end of a line.
        if each_character == &('\n' as u8) {
            y += 1;
            x = 0;
        }  else {
            x += 1;
        }
        current_offset += 1;
    }
    current_offset
}

fn get_width_of_line(message: &String, pos_y: usize) -> usize {
    let mut x: usize = 0;
    let mut y: usize = 0;
    for each_character in message.as_bytes() {
        if y == pos_y + 1 {
            break;
        }
        if each_character == &('\n' as u8) {
            if y == pos_y {
                return x;
            }
            y += 1;
            x = 0;
        } else {
            x += 1;
        }
    }
    x
}

fn get_number_of_lines(message: &String) -> usize {
    let mut num_lines: usize = 0;
    for each_character in message.as_bytes() {
        if each_character == &('\n' as u8) {
            num_lines += 1;
        }
    }
    num_lines
}

fn insert_string(original: &String, insert: &String, pos: usize) -> String {
    let mut original_vec: Vec<char> = original.chars().collect();
    let insert_vec: Vec<char> = insert.chars().collect();
    original_vec.splice(pos..pos, insert_vec);
    original_vec.into_iter().collect()
}

#[derive(Debug, Clone)]
pub struct EditorState {
    piece_table: PieceTable,
    original_buffer: String,
    add_buffer: String,
    running_buffer: String,
    display_buffer: String,
    cursor_state: CursorState,
    line_offset: usize,
    insert_index: usize,
    printable_height: usize,
    printable_width: usize,
    line_wrap: bool,
    quit: bool,
}

fn move_cursor_left(mut editor_state: EditorState) -> EditorState {
    // first we have to commit the working buffer to the piece table
    if editor_state.running_buffer.len() > 0 {
        (editor_state.add_buffer, editor_state.piece_table) = insert_table(editor_state.add_buffer, editor_state.piece_table, &editor_state.running_buffer, editor_state.insert_index);
        // update the insert position to the *end* of the editor_state.running_buffer
        editor_state.insert_index += editor_state.running_buffer.len();
        editor_state.running_buffer = "".to_string();
    }
    if editor_state.insert_index > 0 {
        editor_state.insert_index -= 1;
    }
    // now we can actually update the cursor and related variables
    if editor_state.cursor_state.x == 0 {
        if editor_state.cursor_state.y > 0 {
            (editor_state.cursor_state.x, editor_state.cursor_state.y) = get_position_of_offset(&read_table(&editor_state.piece_table, &editor_state.original_buffer, &editor_state.add_buffer), editor_state.insert_index, editor_state.printable_width, editor_state.line_wrap);
            // don't forget to take pagination into consideration. Absolute length may not be the real height on screen
            editor_state.cursor_state.y -= editor_state.line_offset;
            editor_state.cursor_state.desired_x = editor_state.cursor_state.x;
        } else {
            // don't try to move beyond the start of the document
        }
    }  else {
        // moving the cursor on the current line is easy
        editor_state.cursor_state.x -= 1;
        editor_state.cursor_state.desired_x = editor_state.cursor_state.x;
        editor_state.cursor_state.clip_right = false;
    }
    editor_state
}

fn move_cursor_right(mut editor_state: EditorState) -> EditorState {
    // first we have to commit the working buffer to the piece table
    if editor_state.running_buffer.len() > 0 {
        (editor_state.add_buffer, editor_state.piece_table) = insert_table(editor_state.add_buffer, editor_state.piece_table, &editor_state.running_buffer, editor_state.insert_index);
        // update the insert position to the *end* of the editor_state.running_buffer
        if editor_state.insert_index < read_table(&editor_state.piece_table, &editor_state.original_buffer, &editor_state.add_buffer).len() + editor_state.running_buffer.len() {
            editor_state.insert_index += editor_state.running_buffer.len();
        }
        editor_state.running_buffer = "".to_string();
    }
    // now we can actually update the cursor and related variables
    if editor_state.cursor_state.x == get_width_of_line(&read_table(&editor_state.piece_table, &editor_state.original_buffer, &editor_state.add_buffer), editor_state.cursor_state.y + editor_state.line_offset) {
        if editor_state.insert_index + 1 < read_table(&editor_state.piece_table, &editor_state.original_buffer, &editor_state.add_buffer).len() {
            editor_state.insert_index += 1;
            (editor_state.cursor_state.x, editor_state.cursor_state.y) = get_position_of_offset(&read_table(&editor_state.piece_table, &editor_state.original_buffer, &editor_state.add_buffer), editor_state.insert_index, editor_state.printable_width, editor_state.line_wrap);
            // don't forget to take pagination into consideration. Absolute length may not be the real height on screen
            editor_state.cursor_state.y -= editor_state.line_offset;
            editor_state.cursor_state.desired_x = editor_state.cursor_state.x;
        } else {
            // do nothing when you're at the end of the last line
        }
    } else {
        // moving the cursor on the current line is easy
        editor_state.insert_index += 1;
        editor_state.cursor_state.x += 1;
        editor_state.cursor_state.desired_x = editor_state.cursor_state.x;
        editor_state.cursor_state.clip_right = false;
    }
    editor_state
}

fn move_cursor_down(mut editor_state: EditorState) -> EditorState {
    // first we have to commit the working buffer to the piece table
    if editor_state.running_buffer.len() > 0 {
        (editor_state.add_buffer, editor_state.piece_table) = insert_table(editor_state.add_buffer, editor_state.piece_table, &editor_state.running_buffer, editor_state.insert_index);
        // update the insert position to the *end* of the editor_state.running_buffer
        if editor_state.insert_index < read_table(&editor_state.piece_table, &editor_state.original_buffer, &editor_state.add_buffer).len() + editor_state.running_buffer.len() {
            editor_state.insert_index += editor_state.running_buffer.len();
        }
        editor_state.running_buffer = "".to_string();
    }
    // now we can actually update the cursor and related variables
    if editor_state.cursor_state.y + editor_state.line_offset < get_number_of_lines(&read_table(&editor_state.piece_table, &editor_state.original_buffer, &editor_state.add_buffer)) {
        // clip to end of line if we're at the end of a line
        if editor_state.cursor_state.x == get_width_of_line(&read_table(&editor_state.piece_table, &editor_state.original_buffer, &editor_state.add_buffer), editor_state.cursor_state.y) {
            editor_state.cursor_state.clip_right = true;
        }
        // figure out where to jump in the line below
        let length_of_below_line = get_width_of_line(&read_table(&editor_state.piece_table, &editor_state.original_buffer, &editor_state.add_buffer), editor_state.cursor_state.y + editor_state.line_offset + 1);
        // if at end of a line, go to end of below line
        if editor_state.cursor_state.clip_right || editor_state.cursor_state.x >= length_of_below_line {
            // TODO: factor in line offset and pagination here
            editor_state.cursor_state.y += 1;
            editor_state.cursor_state.x = length_of_below_line;
            editor_state.cursor_state.desired_x = editor_state.cursor_state.x;
            editor_state.insert_index = get_offset_of_position(&read_table(&editor_state.piece_table, &editor_state.original_buffer, &editor_state.add_buffer), editor_state.cursor_state.x, editor_state.cursor_state.y, editor_state.printable_width, editor_state.line_wrap);
        // if in middle of a line, go to same position in above line
        } else {
            // TODO: factor in line offset and pagination here
            editor_state.cursor_state.y += 1;
            // editor_state.cursor_state.x stays the same because the above line is longer than this line
            editor_state.cursor_state.desired_x = editor_state.cursor_state.x;
            editor_state.cursor_state.clip_right = false;
            editor_state.insert_index = get_offset_of_position(&read_table(&editor_state.piece_table, &editor_state.original_buffer, &editor_state.add_buffer), editor_state.cursor_state.x, editor_state.cursor_state.y, editor_state.printable_width, editor_state.line_wrap);
        }
    } else {
        // do nothing, we can't go any further up in the document
    }
    editor_state
}

fn move_cursor_up(mut editor_state: EditorState) -> EditorState {
    // first we have to commit the working buffer to the piece table
    if editor_state.running_buffer.len() > 0 {
        (editor_state.add_buffer, editor_state.piece_table) = insert_table(editor_state.add_buffer, editor_state.piece_table, &editor_state.running_buffer, editor_state.insert_index);
        // update the insert position to the *end* of the editor_state.running_buffer
        if editor_state.insert_index < read_table(&editor_state.piece_table, &editor_state.original_buffer, &editor_state.add_buffer).len() + editor_state.running_buffer.len() {
            editor_state.insert_index += editor_state.running_buffer.len();
        }
        editor_state.running_buffer = "".to_string();
    }
    // now we can actually update the cursor and related variables
    if editor_state.cursor_state.y + editor_state.line_offset > 0 {
        // clip to end of line if we're at the end of a line
        if editor_state.cursor_state.x == get_width_of_line(&read_table(&editor_state.piece_table, &editor_state.original_buffer, &editor_state.add_buffer), editor_state.cursor_state.y) {
            editor_state.cursor_state.clip_right = true;
        }
        // figure out where to jump in the line above
        let length_of_above_line = get_width_of_line(&read_table(&editor_state.piece_table, &editor_state.original_buffer, &editor_state.add_buffer), editor_state.cursor_state.y + editor_state.line_offset - 1);
        // if at end of a line, go to end of above line
        if editor_state.cursor_state.clip_right || editor_state.cursor_state.x >= length_of_above_line {
            // TODO: factor in line offset and pagination here
            editor_state.cursor_state.y -= 1;
            editor_state.cursor_state.x = length_of_above_line;
            editor_state.cursor_state.desired_x = editor_state.cursor_state.x;
            editor_state.insert_index = get_offset_of_position(&read_table(&editor_state.piece_table, &editor_state.original_buffer, &editor_state.add_buffer), editor_state.cursor_state.x, editor_state.cursor_state.y, editor_state.printable_width, editor_state.line_wrap);
        // if in middle of a line, go to same position in above line
        } else {
            // TODO: factor in line offset and pagination here
            editor_state.cursor_state.y -= 1;
            // editor_state.cursor_state.x stays the same because the above line is longer than this line
            editor_state.cursor_state.desired_x = editor_state.cursor_state.x;
            editor_state.cursor_state.clip_right = false;
            editor_state.insert_index = get_offset_of_position(&read_table(&editor_state.piece_table, &editor_state.original_buffer, &editor_state.add_buffer), editor_state.cursor_state.x, editor_state.cursor_state.y, editor_state.printable_width, editor_state.line_wrap);
        }
    } else {
        // do nothing, we can't go any further up in the document
    }
    editor_state
}

fn process_text_input(mut editor_state: EditorState, c: KeyCode, _m: KeyModifiers) -> EditorState {
    // catch-all for spaces, newlines, and characters to add to the buffer
    // keep track of where we started typing on the screen. You can't insert by the cursor position
    // because the cursor will move as you type, but we're not committing each character to the
    // piece table one at a time.
    if editor_state.running_buffer.len() == 0 {
        editor_state.insert_index = get_offset_of_position(&read_table(&editor_state.piece_table, &editor_state.original_buffer, &editor_state.add_buffer), editor_state.cursor_state.x, editor_state.cursor_state.y + editor_state.line_offset, editor_state.printable_width, editor_state.line_wrap);
    }
    match c {
        KeyCode::Char(' ') => {
            editor_state.running_buffer.push(' ');
            (editor_state.add_buffer, editor_state.piece_table) = insert_table(editor_state.add_buffer, editor_state.piece_table, &editor_state.running_buffer, editor_state.insert_index);
            editor_state.running_buffer = "".to_string();
            editor_state.cursor_state.x += 1;
            editor_state.cursor_state.desired_x = editor_state.cursor_state.x;
        },
        KeyCode::Enter => {
            editor_state.running_buffer.push('\n');
            (editor_state.add_buffer, editor_state.piece_table) = insert_table(editor_state.add_buffer, editor_state.piece_table, &editor_state.running_buffer, editor_state.insert_index);
            editor_state.insert_index += editor_state.running_buffer.len();
            editor_state.running_buffer = "".to_string();
            editor_state.cursor_state.desired_x = 0;
            editor_state.cursor_state.x = 0;
            editor_state.cursor_state.y += 1;
        },
        KeyCode::Char(c) => {
            editor_state.running_buffer.push(c);
            editor_state.cursor_state.desired_x += 1;
            editor_state.cursor_state.x += 1;
        },
        _ => (),
    }
    editor_state
}

fn update_editor_state(mut editor_state: EditorState) -> EditorState {
    let mut stdout = stdout();
    enable_raw_mode().unwrap();
    match read().unwrap() {
        Event::Key(KeyEvent {
            code: KeyCode::Char('h'),
            modifiers: KeyModifiers::CONTROL,
        }) => execute!(stdout, Clear(ClearType::All), Print("This is a minimalist text editor.")).unwrap(),
        Event::Key(KeyEvent {
            code: KeyCode::Char('k'),
            modifiers: KeyModifiers::ALT,
        }) => execute!(stdout, Clear(ClearType::All), Print("You typed alt-k")).unwrap(),
        Event::Key(KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::CONTROL,
        }) => editor_state.quit = true,
        Event::Key(KeyEvent {
            code: KeyCode::Left,
            modifiers: _,
        }) => {
            editor_state = move_cursor_left(editor_state);
        },
        Event::Key(KeyEvent {
            code: KeyCode::Right,
            modifiers: _,
        }) => {
            editor_state = move_cursor_right(editor_state);
        },
        Event::Key(KeyEvent {
            code: KeyCode::Up,
            modifiers: _,
        }) => {
            editor_state = move_cursor_up(editor_state);
        },
        Event::Key(KeyEvent {
            code: KeyCode::Down,
            modifiers: _,
        }) => {
            editor_state = move_cursor_down(editor_state);
        },
        Event::Key(KeyEvent {
            code: c,
            modifiers: m
        }) => {
            editor_state = process_text_input(editor_state, c, m);
        },
        _ => (),
    }
    disable_raw_mode().unwrap();
    editor_state
}

fn save_editor_states(state_history: Vec<EditorState>) {
    let out_file = File::create("editor_state_history.txt").unwrap();
    let mut writer = BufWriter::new(out_file);

    for state in state_history {
        writeln!(writer, "{:?}\n\n", state).unwrap();
    }
}

fn split_lines_wrapped(input_text: String, screen_height: usize, line_width: usize, line_wrap: bool) -> String {
    /*
     * This function takes a string represents the screen to display. It then 
     * returns a string that can be printed to show the whole, line-wrapped screen.
     * This involves splitting the screen by the newlines already built into it,
     * then splitting each of those into multiple lines, if needed, 
     * due to line wrapping.
     */
    let mut lines: Vec<String> = Vec::new(); 
    let mut line_iter = input_text.as_bytes().split(|c| c == &('\n' as u8));
    loop {
        match line_iter.next() {
            Some(v) => {
                // blank lines are special cases, put an empty string in the vector so we don't lose them
                if v.len() == 0 {
                    lines.push("".to_string());
                }
                // no line wrapping, don't do anything fancy
                if !line_wrap {
                    lines.push(std::str::from_utf8(v).unwrap().to_string());
                }else {
                    // line wrapping, split each line into chunks
                    for each_line in v.chunks(line_width) {
                        lines.push(std::str::from_utf8(each_line).unwrap().to_string())
                    }
                }
            },
            _ => break,
        }
    }
    // combine all the lines into the new line-wrapped version
    if lines.len() > screen_height {
        lines = lines.split_at(screen_height).0.to_vec();
    }
    lines.join("\n")
}

fn render_editor(editor_state: &EditorState) {
    /*
     * The function that accepts the current EditorState and displays it on the screen.
     * This assumes that editor_state.display_buffer is the full contents of the document, 
     * including the text being inserted which hasn't been committted to the piece table yet.
     * Based on that, it performs pagination and line-wrapping (if applicable), then renders 
     * the result.
     */
    if editor_state.line_offset > get_number_of_lines(&editor_state.display_buffer) {
        return;
    }
    // use editor_state.line_offset to determine how many lines into the document the 
    // first displayed line should be 
    let start_of_page: usize = get_offset_of_position(&editor_state.display_buffer, 0, editor_state.line_offset, editor_state.printable_width, editor_state.line_wrap);
    let end_of_page: usize = get_offset_of_position(&editor_state.display_buffer, 0, editor_state.line_offset + editor_state.printable_height, editor_state.printable_width, editor_state.line_wrap);
    // read the display_buffer from start_of_page to end_of_page
    // TODO - make sure this isn't off by one
    let mut paginated_display = String::from_utf8(editor_state.display_buffer.as_bytes().split_at(start_of_page).1.to_vec()).unwrap();
    // cut off the display_buffer at the end of the page
    // TODO - make sure this isn't off by one
    paginated_display = String::from_utf8(paginated_display.as_bytes().split_at(end_of_page).0.to_vec()).unwrap();
    // TODO - perform any line-wrapping now
    paginated_display = split_lines_wrapped(paginated_display, editor_state.printable_height, editor_state.printable_width, editor_state.line_wrap);
    // display the paginated, line-wrapped screen
    println!("{}", paginated_display);
}

fn main() {
    let piece_table: PieceTable = PieceTable{
        which: Vec::new(),
        start: Vec::new(),
        end: Vec::new(),
    };
    let original_buffer: String = "".to_string();
    let add_buffer: String = "".to_string();
    let running_buffer: String = "".to_string();
    let display_buffer: String = "".to_string();
    let cursor_state: CursorState = CursorState{
        desired_x: 0,
        x: 0,
        y: 0,
        clip_right: false,
    };
    let line_offset: usize = 0;
    let mut stdout = stdout();
    enable_raw_mode().unwrap();
    let insert_index: usize = 0;

    let mut editor_state = EditorState{
        piece_table: piece_table,
        original_buffer: original_buffer,
        add_buffer: add_buffer,
        running_buffer: running_buffer,
        display_buffer: display_buffer,
        cursor_state: cursor_state,
        line_offset: line_offset,
        insert_index: insert_index,
        line_wrap: true,
        printable_height: 24,
        printable_width: 80,
        quit: false,
    };

    let mut state_history: Vec<EditorState> = Vec::new();

    loop {
        execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
        println!("{}", make_text_green(&"Welcome to Goncharov!".to_string()));
        editor_state.display_buffer = read_table(&editor_state.piece_table, &editor_state.original_buffer, &editor_state.add_buffer);
        //print!("display buffer before insert: {}", editor_state.display_buffer);
        editor_state.display_buffer = insert_string(&editor_state.display_buffer, &editor_state.running_buffer, editor_state.insert_index);
        //println!("display buffer after  insert: {}", editor_state.display_buffer);
        //println!("cursor position: ({}, {})", editor_state.cursor_state.x, editor_state.cursor_state.y);
        render_editor(&editor_state);
        execute!(stdout, cursor::MoveTo(editor_state.cursor_state.x as u16, (editor_state.cursor_state.y - editor_state.line_offset + 1) as u16)).unwrap();
        editor_state = update_editor_state(editor_state);
        // cache the current EditorState for extreme debugging
        state_history.push(editor_state.clone());
        if editor_state.quit {
            break;
        }
    }
    print!("\n\n");
    println!("{}", read_table(&editor_state.piece_table, &editor_state.original_buffer, &editor_state.add_buffer));
    println!("Original buffer: {}", editor_state.original_buffer);
    println!("Add buffer: {}", editor_state.add_buffer);
    println!("{}", editor_state.piece_table);

    // save the history of EditorStates to a file
    save_editor_states(state_history);

    disable_raw_mode().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_lines_wrapped_01() {
        // 81 characters long
        let input_text: String = "012345678901234567890123456789012345678901234567890123456789012345678901234567890".to_string();
        // should only wrap one character
        let output_text: String = "01234567890123456789012345678901234567890123456789012345678901234567890123456789\n0".to_string();
        assert_eq!(split_lines_wrapped(input_text, 24, 80, true), output_text);
    }

    #[test]
    fn test_split_lines_wrapped_02() {
        // line is 161 characters long
        let input_text: String = "01234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890".to_string();
        // should wrap twice
        let output_text: String = "01234567890123456789012345678901234567890123456789012345678901234567890123456789\n01234567890123456789012345678901234567890123456789012345678901234567890123456789\n0".to_string();
        assert_eq!(split_lines_wrapped(input_text, 24, 80, true), output_text);
    }

    #[test]
    fn test_split_lines_wrapped_03() {
        // longer, more complicated input with blank lines, short lines, etc
        let input_text: String = "01234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890\naaa\n\n\n012345678901234567890123456789012345678901234567890123456789012345678901234567890\na\na\na".to_string();
        let output_text: String = "01234567890123456789012345678901234567890123456789012345678901234567890123456789\n01234567890123456789012345678901234567890123456789012345678901234567890123456789\n0\naaa\n\n\n01234567890123456789012345678901234567890123456789012345678901234567890123456789\n0\na\na\na".to_string();
        assert_eq!(split_lines_wrapped(input_text, 24, 80, true), output_text);
    }

    #[test]
    fn test_get_position_of_offset_01() {
        let message = "a\nb\n\naaa\naaa".to_string();
        assert_eq!(get_position_of_offset(&message, 0, 80, false), (0, 0));
    }

    #[test]
    fn test_get_position_of_offset_02() {
        let message = "a\nb\n\naaa\naaa".to_string();
        assert_eq!(get_position_of_offset(&message, 1, 80, false), (1, 0));
    }

    #[test]
    fn test_get_position_of_offset_03() {
        let message = "a\nb\n\naaa\naaa".to_string();
        assert_eq!(get_position_of_offset(&message, 2, 80, false), (0, 1));
    }

    #[test]
    fn test_get_position_of_offset_04() {
        let message = "a\nb\n\naaa\naaa".to_string();
        assert_eq!(get_position_of_offset(&message, 5, 80, false), (0, 3));
    }

    #[test]
    fn test_get_position_of_offset_05() {
        let message = "a\nb\n\naaa\naaa".to_string();
        assert_eq!(get_position_of_offset(&message, 8, 80, false), (3, 3));
    }

    #[test]
    fn test_get_position_of_offset_06() {
        let message = "a\nb\n\naaa\naaa".to_string();
        assert_eq!(get_position_of_offset(&message, 9, 80, false), (0, 4));
    }

    #[test]
    fn test_get_position_of_offset_line_wrap_01() {
        // a single line that is 79 characters long shouldn't wrap
        let message = "0123456789012345678901234567890123456789012345678901234567890123456789012345678".to_string();
        assert_eq!(get_position_of_offset(&message, 79, 80, true), (79, 0));
    }

    #[test]
    fn test_get_position_of_offset_line_wrap_02() {
        // a single line that is 80 characters long won't wrap any letters, 
        // but the if you move the cursor to the end, it will go to the start of the next line.
        let message = "01234567890123456789012345678901234567890123456789012345678901234567890123456789".to_string();
        assert_eq!(get_position_of_offset(&message, 80, 80, true), (0, 1));
    }

    #[test]
    fn test_get_position_of_offset_line_wrap_03() {
        // a single line that is 81 characters should wrap
        let message = "012345678901234567890123456789012345678901234567890123456789012345678901234567890".to_string();
        assert_eq!(get_position_of_offset(&message, 81, 80, true), (1, 1));
    }

    #[test]
    fn test_get_offset_of_position_01() {
        let message = "a\nb\n\naaa\naaa".to_string();
        assert_eq!(get_offset_of_position(&message, 0, 0, 80, false), 0);
    }

    #[test]
    fn test_get_offset_of_position_02() {
        let message = "a\nb\n\naaa\naaa".to_string();
        assert_eq!(get_offset_of_position(&message, 1, 0, 80, false), 1);
    }

    #[test]
    fn test_get_offset_of_position_03() {
        let message = "a\nb\n\naaa\naaa".to_string();
        assert_eq!(get_offset_of_position(&message, 0, 1, 80, false), 2);
    }

    #[test]
    fn test_get_offset_of_position_04() {
        let message = "a\nb\n\naaa\naaa".to_string();
        assert_eq!(get_offset_of_position(&message, 0, 3, 80, false), 5);
    }

    #[test]
    fn test_get_offset_of_position_05() {
        let message = "a\nb\n\naaa\naaa".to_string();
        assert_eq!(get_offset_of_position(&message, 3, 3, 80, false), 8);
    }

    #[test]
    fn test_get_offset_of_position_06() {
        let message = "a\nb\n\naaa\naaa".to_string();
        assert_eq!(get_offset_of_position(&message, 0, 4, 80, false), 9);
    }

    #[test]
    fn test_get_offset_of_line_wrap_position_01() {
        // a single line that is 81 characters should wrap
        let message = "012345678901234567890123456789012345678901234567890123456789012345678901234567890".to_string();
        assert_eq!(get_offset_of_position(&message, 1, 1, 80, true), 80);
    }

    #[test]
    fn test_get_offset_of_line_wrap_position_02() {
        // a single line that is 81 characters should wrap
        let message = "012345678901234567890123456789012345678901234567890123456789012345678901234567890".to_string();
        assert_eq!(get_offset_of_position(&message, 0, 79, 80, true), 79);
    }
}
