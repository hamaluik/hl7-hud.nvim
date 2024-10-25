use nvim_oxi::{api, Dictionary, Function, Object};

#[nvim_oxi::plugin]
fn hl7_tools() -> Dictionary {
    Dictionary::from_iter([
        (
            "locate_cursor",
            Object::from(Function::from_fn(locate_cursor)),
        ),
        ("goto_query", Object::from(Function::from_fn(goto_query))),
    ])
}

fn get_buffer_content(buffer: &api::Buffer) -> Result<String, String> {
    let lines = match buffer.get_lines(0..buffer.line_count().unwrap_or_default(), false) {
        Ok(lines) => lines,
        Err(e) => return Err(format!("Error: {e:#}")),
    };
    Ok(lines
        .map(|l| l.to_string())
        .collect::<Vec<String>>()
        .join("\r"))
}

fn locate_cursor(_: ()) -> String {
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

fn goto_query(query: String) {
    let query = match hl7_parser::query::parse_location_query(query.to_ascii_uppercase().as_str()) {
        Ok(query) => query,
        Err(e) => {
            api::err_writeln(&format!("Error parsing query: {e:#}"));
            return;
        }
    };

    let mut window = api::get_current_win();
    let buffer = match window.get_buf() {
        Ok(buf) => buf,
        Err(e) => {
            api::err_writeln(&format!("Error getting buffer: {e:#}"));
            return;
        }
    };
    let content = match get_buffer_content(&buffer) {
        Ok(content) => content,
        Err(e) => {
            api::err_writeln(&format!("Error getting buffer content: {e:#}"));
            return;
        }
    };

    let message = match hl7_parser::parse_message(content.as_str()) {
        Ok(message) => message,
        Err(e) => {
            api::err_writeln(&format!("Error parsing message: {e:#}"));
            return;
        }
    };

    let Some(result) = message.query(query) else {
        api::err_writeln("Query did not match any location in the message");
        return;
    };

    let range = result.range();

    // calculate the line number and column number
    // search for it because nvim-oxi doesn't have an API for byte2line
    let num_lines = buffer.line_count().unwrap_or_default();

    for i in 0..num_lines {
        let line_start = buffer.get_offset(i).unwrap_or_default();
        let next_line_start = buffer.get_offset(i + 1).unwrap_or_default();
        if line_start <= range.start && range.start < next_line_start {

            let line = i + 1;
            let col = range.start - line_start;
            window.set_cursor(line, col).unwrap_or_else(|e| {
                api::err_writeln(&format!("Error setting cursor: {e:#}"));
            });
            return;
        }
    }

    api::err_writeln("Could not find the location in the buffer");
}

