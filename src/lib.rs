use nvim_oxi::{api, Dictionary, Function, Object};

mod goto_query;
mod locate_cursor;
mod parse_timestamp;
mod generate;

#[nvim_oxi::plugin]
fn hl7_tools() -> Dictionary {
    Dictionary::from_iter([
        (
            "locate_cursor",
            Object::from(Function::from_fn(locate_cursor::locate_cursor)),
        ),
        (
            "goto_query",
            Object::from(Function::from_fn(goto_query::goto_query)),
        ),
        (
            "parse_timestamp",
            Object::from(Function::from_fn(parse_timestamp::parse_timestamp)),
        ),
        (
            "parse_timestamp_at_cursor",
            Object::from(Function::from_fn(
                parse_timestamp::parse_timestamp_at_cursor,
            )),
        ),
        (
            "generate_timestamp",
            Object::from(Function::from_fn(generate::generate_timestamp)),
        ),
        (
            "generate_control_id",
            Object::from(Function::from_fn(generate::generate_control_id)),
        ),
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
