#[cfg(test)]
mod tests {
    use crate::*;

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

    #[test]
    fn test_move_cursor_left_01() {
        let initial_editor_state: EditorState = EditorState {
            piece_table: PieceTable{
                which: Vec::new(),
                start: Vec::new(),
                end: Vec::new(),
            },
            original_buffer: "".to_string(),
            add_buffer: "".to_string(),
            running_buffer: "".to_string(),
            display_buffer: "".to_string(),
            cursor_state: CursorState{
                desired_x: 0,
                x: 0,
                y: 0,
                clip_right: false,
            },
            line_offset: 0,
            insert_index: 0,
            line_wrap: true,
            printable_height: 24,
            printable_width: 80,
            quit: false,
        };

        let after_editor_state = move_cursor_left(initial_editor_state);
        assert_eq!(after_editor_state.cursor_state.x, 0);
        assert_eq!(after_editor_state.cursor_state.y, 0);
    }
}