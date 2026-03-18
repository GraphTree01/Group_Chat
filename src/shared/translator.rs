use std::io::{self, BufRead, Write};

use crate::shared::protocol::Message;

pub fn to_json(message: &Message) -> Result<String, serde_json::Error> {
	serde_json::to_string(message)
}

pub fn from_json(json: &str) -> Result<Message, serde_json::Error> {
	serde_json::from_str(json)
}

pub fn write_message_line(writer: &mut impl Write, message: &Message) -> io::Result<()> {
	let json = to_json(message).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
	writeln!(writer, "{json}")
}

pub fn read_message_line(
	reader: &mut impl BufRead,
	buffer: &mut String,
) -> io::Result<Option<Message>> {
	buffer.clear();

	let bytes_read = reader.read_line(buffer)?;
	if bytes_read == 0 {
		return Ok(None);
	}

	let trimmed = buffer.trim();
	if trimmed.is_empty() {
		return Ok(None);
	}

	let message = from_json(trimmed)
		.map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;

	Ok(Some(message))
}

#[cfg(test)]
mod tests {
	use std::io::{BufReader, Cursor};

	use super::*;

	#[test]
	fn serializes_and_deserializes_message() {
		let original = Message::identify("Kimberly");
		let json = to_json(&original).expect("serialize message");
		let parsed = from_json(&json).expect("deserialize message");

		match parsed {
			Message::Identify { username } => assert_eq!(username, "Kimberly"),
			_ => panic!("unexpected message variant"),
		}
	}

	#[test]
	fn writes_json_with_trailing_newline() {
		let message = Message::new_user("Kimberly");
		let mut output = Vec::new();

		write_message_line(&mut output, &message).expect("write message line");

		let as_text = String::from_utf8(output).expect("valid utf8 output");
		assert_eq!(as_text, "{\"type\":\"NEW_USER\",\"username\":\"Kimberly\"}\n");
	}

	#[test]
	fn reads_single_message_line() {
		let input = Cursor::new("{\"type\":\"IDENTIFY\",\"username\":\"Kimberly\"}\n");
		let mut reader = BufReader::new(input);
		let mut buffer = String::new();

		let parsed = read_message_line(&mut reader, &mut buffer)
			.expect("read message line")
			.expect("message should exist");

		match parsed {
			Message::Identify { username } => assert_eq!(username, "Kimberly"),
			_ => panic!("unexpected message variant"),
		}
	}
}
