#[macro_use]
extern crate crossterm;

use std::{fmt};
use std::io::{self, stdout, Write};
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

#[derive(Debug)]
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

#[derive(Debug)]
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

fn main() {
    let mut piece_table: PieceTable = PieceTable{
        which: Vec::new(),
        start: Vec::new(),
        end: Vec::new(),
    };
    let mut original_buffer: String = "".to_string();
    let mut add_buffer: String = "".to_string();
    let mut running_buffer: String = "".to_string();
    let mut cursor_state: CursorState = CursorState{
        desired_x: 0,
        x: 0,
        y: 1,
        clip_right: false,
    };
    let mut line_offset: usize = 0;
    let mut stdout = stdout();
    enable_raw_mode().unwrap();

    loop {
        execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
        println!("{}", make_text_green(&"Welcome to Goncharov!".to_string()));
        print!("{}", read_table(&piece_table, &original_buffer, &add_buffer));
        println!("{}", running_buffer);
        execute!(stdout, cursor::MoveTo(cursor_state.x as u16, (cursor_state.y - line_offset) as u16)).unwrap();
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
            }) => break,
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                modifiers: _,
            }) => {
                // first we have to commit the working buffer to the piece table
                if running_buffer.len() > 0 {
                    let insert_index: usize = get_offset_of_position(&read_table(&piece_table, &original_buffer, &add_buffer), cursor_state.x, cursor_state.y + line_offset);
                    (add_buffer, piece_table) = insert_table(add_buffer, piece_table, &running_buffer, insert_index);
                    running_buffer = "".to_string();
                }
                // now we can actually update the cursor and related variables
                if cursor_state.x == 0 {
                    if cursor_state.y > 0 {
                        let offset: usize = get_offset_of_position(&read_table(&piece_table, &original_buffer, &add_buffer), cursor_state.x, cursor_state.y + line_offset) - 1;
                        (cursor_state.x, cursor_state.y) = get_position_of_offset(&read_table(&piece_table, &original_buffer, &add_buffer), offset);
                        // don't forget to take pagination into consideration. Absolute length may not be the real height on screen
                        cursor_state.y -= line_offset;
                        cursor_state.desired_x = cursor_state.x;
                        cursor_state.clip_right = true;
                    }
                }else {
                    cursor_state.x -= 1;
                    cursor_state.desired_x = cursor_state.x;
                    cursor_state.clip_right = false;
                }
            },
            Event::Key(KeyEvent {
                code: c,
                modifiers: m
            }) => {
                match c {
                    KeyCode::Char(' ') => {
                        let insert_index: usize = get_offset_of_position(&read_table(&piece_table, &original_buffer, &add_buffer), cursor_state.x, cursor_state.y + line_offset);
                        running_buffer.push(' ');
                        (add_buffer, piece_table) = insert_table(add_buffer, piece_table, &running_buffer, insert_index);
                        running_buffer = "".to_string();
                        cursor_state.desired_x += 1;
                        cursor_state.x += 1;
                    },
                    KeyCode::Enter => {
                        let insert_index: usize = get_offset_of_position(&read_table(&piece_table, &original_buffer, &add_buffer), cursor_state.x, cursor_state.y + line_offset);
                        running_buffer.push('\n');
                        (add_buffer, piece_table) = insert_table(add_buffer, piece_table, &running_buffer, insert_index);
                        running_buffer = "".to_string();
                        cursor_state.desired_x = 0;
                        cursor_state.x = 0;
                        cursor_state.y += 1;
                    },
                    KeyCode::Char(c) => {
                        running_buffer.push(c);
                        cursor_state.desired_x += 1;
                        cursor_state.x += 1;
                    },
                    _ => (),
                }
            },
            _ => (),
        }
    }
    println!("{}", read_table(&piece_table, &original_buffer, &add_buffer));
    println!("Original buffer: {}", original_buffer);
    println!("Add buffer: {}", add_buffer);
    println!("{}", piece_table);

    disable_raw_mode().unwrap();
}
