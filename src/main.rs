#[macro_use]
extern crate crossterm;

use std::fmt;
use std::fs::File;
use std::io::{self, stdout, Write};
//use std::io::prelude::*;
use std::io::BufWriter;
use crossterm::cursor;
use crossterm::event::{self, read, Event, KeyCode, KeyEvent, KeyModifiers};
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

fn test_text() {
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

fn delete_text(mut piece_table: PieceTable, delete_start: usize, delete_end: usize) -> PieceTable {
    

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
        }else {
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

fn get_table_length(piece_table: &PieceTable) -> usize {
    let mut table_length: usize = 0;
    for each_index in 0..piece_table.which.len() {
        table_length += piece_table.end[each_index] - piece_table.start[each_index];
    }
    table_length
}

fn cursor_left(move_by: i32) {
    print!("\x1B[{}D", move_by);
    io::stdout().flush().unwrap();
}

fn cursor_right(move_by: i32) {
    print!("\x1B[{}C", move_by);
    io::stdout().flush().unwrap();
}

fn cursor_up(move_by: i32) {
    print!("\x1B[{}A", move_by);
    io::stdout().flush().unwrap();
}

fn cursor_down(move_by: i32) {
    print!("\x1B[{}B", move_by);
    io::stdout().flush().unwrap();
}

fn make_text_red(text: &String) -> String {
    format!("\x1b[31m{}\x1b[0m", &text)
}

fn make_text_green(text: &String) -> String {
    format!("\x1b[32m{}\x1b[0m", &text)
}

fn make_text_blue(text: &String) -> String {
    format!("\x1b[34m{}\x1b[0m", &text)
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn get_position_of_offset(message: &String, offset: usize) -> (usize, usize) {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut current_offset: usize = 0;
    for each_character in message.as_bytes() {
        if current_offset == offset {
            break;
        }
        if each_character == &('\n' as u8) {
            y += 1;
            x = 0;
        }else {
            x += 1;
        }
        current_offset += 1;
    }
    (x, y)
}

fn get_offset_of_position(message: &String, pos_x: usize, pos_y: usize) -> usize {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut current_offset: usize = 0;
    for each_character in message.as_bytes() {
        if y == pos_y && x == pos_x {
            break;
        }
        if each_character == &('\n' as u8) {
            y += 1;
            x = 0;
        }else {
            x += 1;
        }
        current_offset += 1;
    }
    current_offset
}

fn get_width_of_line(message: &String, pos_y: usize) -> usize {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut last_line_length: usize = 0;
    let mut current_offset: usize = 0;
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
        }else {
            x += 1;
        }
        current_offset += 1;
    }
    x
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
    quit: bool,
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
                    (editor_state.cursor_state.x, editor_state.cursor_state.y) = get_position_of_offset(&read_table(&editor_state.piece_table, &editor_state.original_buffer, &editor_state.add_buffer), editor_state.insert_index);
                    // don't forget to take pagination into consideration. Absolute length may not be the real height on screen
                    editor_state.cursor_state.y -= editor_state.line_offset;
                    editor_state.cursor_state.desired_x = editor_state.cursor_state.x;
                } else {
                    // don't try to move beyond the start of the document
                }
            }else {
                // moving the cursor on the current line is easy
                editor_state.cursor_state.x -= 1;
                editor_state.cursor_state.desired_x = editor_state.cursor_state.x;
            }
        },
        Event::Key(KeyEvent {
            code: KeyCode::Right,
            modifiers: _,
        }) => {
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
                    (editor_state.cursor_state.x, editor_state.cursor_state.y) = get_position_of_offset(&read_table(&editor_state.piece_table, &editor_state.original_buffer, &editor_state.add_buffer), editor_state.insert_index);
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
            }
        },
        Event::Key(KeyEvent {
            code: KeyCode::Up,
            modifiers: _,
        }) => {
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
                let mut length_of_above_line = get_width_of_line(&read_table(&editor_state.piece_table, &editor_state.original_buffer, &editor_state.add_buffer), editor_state.cursor_state.y + editor_state.line_offset - 1);
                // if at end of a line, go to end of above line
                if editor_state.cursor_state.clip_right || editor_state.cursor_state.x >= length_of_above_line {
                    // TODO: factor in line offset and pagination here
                    editor_state.cursor_state.y -= 1;
                    editor_state.cursor_state.x = length_of_above_line;
                    editor_state.cursor_state.desired_x = editor_state.cursor_state.x;
                    editor_state.insert_index = get_offset_of_position(&read_table(&editor_state.piece_table, &editor_state.original_buffer, &editor_state.add_buffer), editor_state.cursor_state.x, editor_state.cursor_state.y);
                // if in middle of a line, go to same position in above line
                } else {
                    // TODO: factor in line offset and pagination here
                    editor_state.cursor_state.y -= 1;
                    // editor_state.cursor_state.x stays the same because the above line is longer than this line
                    editor_state.cursor_state.desired_x = editor_state.cursor_state.x;
                    editor_state.insert_index = get_offset_of_position(&read_table(&editor_state.piece_table, &editor_state.original_buffer, &editor_state.add_buffer), editor_state.cursor_state.x, editor_state.cursor_state.y);
                }
            } else {
                // do nothing, we can't go any further up in the document
            }
        },
        Event::Key(KeyEvent {
            code: c,
            modifiers: m
        }) => {
            // catch-all for spaces, newlines, and characters to add to the buffer
            // keep track of where we started typing on the screen. You can't insert by the cursor position
            // because the cursor will move as you type, but we're not committing each character to the
            // piece table one at a time.
            if editor_state.running_buffer.len() == 0 {
                editor_state.insert_index = get_offset_of_position(&read_table(&editor_state.piece_table, &editor_state.original_buffer, &editor_state.add_buffer), editor_state.cursor_state.x, editor_state.cursor_state.y + editor_state.line_offset);
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

fn main() {
    let mut piece_table: PieceTable = PieceTable{
        which: Vec::new(),
        start: Vec::new(),
        end: Vec::new(),
    };
    let mut original_buffer: String = "".to_string();
    let mut add_buffer: String = "".to_string();
    let mut running_buffer: String = "".to_string();
    let mut display_buffer: String = "".to_string();
    let mut cursor_state: CursorState = CursorState{
        desired_x: 0,
        x: 0,
        y: 0,
        clip_right: false,
    };
    let mut line_offset: usize = 0;
    let mut stdout = stdout();
    enable_raw_mode().unwrap();
    let mut insert_index: usize = 0;

    let mut editor_state = EditorState{
        piece_table: piece_table,
        original_buffer: original_buffer,
        add_buffer: add_buffer,
        running_buffer: running_buffer,
        display_buffer: display_buffer,
        cursor_state: cursor_state,
        line_offset: line_offset,
        insert_index: insert_index,
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
        println!("{}", editor_state.display_buffer);
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
    fn test_get_position_of_offset_01() {
        let message = "a\nb\n\naaa\naaa".to_string();
        assert_eq!(get_position_of_offset(&message, 0), (0, 0));
    }

    #[test]
    fn test_get_position_of_offset_02() {
        let message = "a\nb\n\naaa\naaa".to_string();
        assert_eq!(get_position_of_offset(&message, 1), (1, 0));
    }

    #[test]
    fn test_get_position_of_offset_03() {
        let message = "a\nb\n\naaa\naaa".to_string();
        assert_eq!(get_position_of_offset(&message, 2), (0, 1));
    }

    #[test]
    fn test_get_position_of_offset_04() {
        let message = "a\nb\n\naaa\naaa".to_string();
        assert_eq!(get_position_of_offset(&message, 5), (0, 3));
    }

    #[test]
    fn test_get_position_of_offset_05() {
        let message = "a\nb\n\naaa\naaa".to_string();
        assert_eq!(get_position_of_offset(&message, 8), (3, 3));
    }

    #[test]
    fn test_get_position_of_offset_06() {
        let message = "a\nb\n\naaa\naaa".to_string();
        assert_eq!(get_position_of_offset(&message, 9), (0, 4));
    }

    #[test]
    fn test_get_offset_of_position_01() {
        let message = "a\nb\n\naaa\naaa".to_string();
        assert_eq!(get_offset_of_position(&message, 0, 0), 0);
    }

    #[test]
    fn test_get_offset_of_position_02() {
        let message = "a\nb\n\naaa\naaa".to_string();
        assert_eq!(get_offset_of_position(&message, 1, 0), 1);
    }

    #[test]
    fn test_get_offset_of_position_03() {
        let message = "a\nb\n\naaa\naaa".to_string();
        assert_eq!(get_offset_of_position(&message, 0, 1), 2);
    }

    #[test]
    fn test_get_offset_of_position_04() {
        let message = "a\nb\n\naaa\naaa".to_string();
        assert_eq!(get_offset_of_position(&message, 0, 3), 5);
    }

    #[test]
    fn test_get_offset_of_position_05() {
        let message = "a\nb\n\naaa\naaa".to_string();
        assert_eq!(get_offset_of_position(&message, 3, 3), 8);
    }

    #[test]
    fn test_get_offset_of_position_06() {
        let message = "a\nb\n\naaa\naaa".to_string();
        assert_eq!(get_offset_of_position(&message, 0, 4), 9);
    }
}
