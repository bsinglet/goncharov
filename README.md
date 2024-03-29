# Goncharov

## What is it?
Goncharov is a Rust-based, cross-platform, CLI text editor. It is still in pre-alpha status as it is missing a few key features, such as being able to load a file, save a file, or use the backspace key...

## How to run it?
1. Clone this github repo
2. cd into the new directory
3. Execute `cargo run`

## Open Tasks
- Add code to increment/decrement line_offset in the EditorState updater
- Figure out the right interactions between cursor_state.y and line_offset, then make all their appearances in the code consistent
- Need to clean up update_editor_state, make it more consistent in using editor_state.display_buffer or similar if possible

## Roadmap
### Already Implemented
1. Implementing the piece table data structure to allow for efficiently tracking insertions and deletions on large documents, with infinite undo and redo potential at zero cost
2. Displaying typed text
3. Inserting text into arbitrary positions
4. Breaking the editor state out into a struct for better debugging
5. Left cursor movement
6. Right cursor movement
7. Up cursor movement
8. Down cursor movement

### Upcoming features
9. Pagination
10. Loading files
11. Saving files
12. Deleting text
13. Find-replace functionality
14. CLI arguments
15. LSP support, potentially
16. Scripting support
