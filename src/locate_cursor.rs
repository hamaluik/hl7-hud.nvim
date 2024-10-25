use nvim_oxi::api;

use crate::get_buffer_content;

pub fn locate_cursor(_: ()) -> String {
    let window = api::get_current_win();
    let cursor = match window.get_cursor() {
        Ok(cursor) => cursor,
        Err(e) => {
            api::err_writeln(&format!("Error getting cursor: {e:#}"));
            return "".to_string();
        }
    };

    let buffer = match window.get_buf() {
        Ok(buf) => buf,
        Err(e) => {
            api::err_writeln(&format!("Error getting buffer: {e:#}"));
            return "".to_string();
        }
    };

    let line_offset = match buffer.get_offset(cursor.0 - 1) {
        Ok(offset) => offset,
        Err(e) => {
            api::err_writeln(&format!("Error getting line offset: {e:#}"));
            return "".to_string();
        }
    };
    let buffer_offset = line_offset + cursor.1 as usize + 1;

    let buffer = match get_buffer_content(&buffer) {
        Ok(content) => content,
        Err(e) => {
            api::err_writeln(&format!("Error getting buffer content: {e:#}"));
            return "".to_string();
        }
    };

    let message = match hl7_parser::parse_message(buffer.as_str()) {
        Ok(message) => message,
        Err(e) => {
            api::err_writeln(&format!("Error parsing message: {e:#}"));
            return "".to_string();
        }
    };

    let Some(location) = message.locate_cursor(buffer_offset) else {
        return "?".to_string();
    };

    format!("{location}")
}

