use chrono::{DateTime, Local};
use nvim_oxi::api;
use crate::get_buffer_content;

pub fn parse_timestamp(timestamp: String) -> String {
    let timestamp = timestamp.trim();
    let timestamp = match hl7_parser::timestamps::parse_timestamp(timestamp) {
        Ok(ts) => ts,
        Err(e) => {
            api::err_writeln(&format!("Error parsing timestamp: {:#?}", e));
            return "".to_string();
        }
    };

    let timestamp: DateTime<Local> = match timestamp.try_into() {
        Ok(ts) => ts,
        Err(e) => {
            api::err_writeln(&format!("Error converting timestamp to local time: {:#?}", e));
            return "".to_string();
        }
    };

    timestamp.to_rfc2822()
}

pub fn parse_timestamp_at_cursor(_: ()) -> String {
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
        api::err_writeln("?");
        return "".to_string();
    };

    let value = if let Some((_, sub_component)) = location.sub_component {
        sub_component.raw_value()
    }
    else if let Some((_, component)) = location.component {
        component.raw_value()
    }
    else if let Some((_, repeat)) = location.repeat {
        repeat.raw_value()
    }
    else if let Some((_, field)) = location.field {
        field.raw_value()
    }
    else if let Some((_, _, segment)) = location.segment {
        segment.raw_value()
    }
    else {
        api::err_writeln("Cursor is not in a message?");
        return "".to_string();
    };

    return parse_timestamp(value.to_string());
}

