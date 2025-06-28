use {
	std::{error::Error, fs, io::Write},
	webp::Decoder,
};

fn main() -> Result<(), Box<dyn Error>> {
	let input_file: &str = "assets/icon.webp";
	let output_file: &str = "assets/icon-256*256.rgba"; // TODO: remove hard code

	fs::File::create(output_file)?.write_all(
		&Decoder::new(&fs::read(input_file)?)
			.decode()
			.ok_or("Failed to decode WebPImage")?
			.as_ref(),
	)?;

	Ok(())
}
