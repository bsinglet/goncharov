use std::fmt;

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
        for each_entry in &self.which {
            result += &format!("{}, ", each_entry).as_str();
        }
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

fn main() {
    test_text();
}
