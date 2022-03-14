use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::{Error, Result};
use std::fs;

use crate::chunk::Chunk;
use crate::png::Png;
use std::fs::{read, write};
use std::path::PathBuf;

pub fn encode(
    EncodeArgs {
        file_path,
        chunk_type,
        message,
        output_file,
    }: EncodeArgs,
) -> Result<()> {
    let mut png = read_png(&file_path)?;

    let new_chunk = Chunk::new(chunk_type, message.as_bytes().to_vec());
    png.append_chunk(new_chunk);

    if let Some(output_file) = output_file {
        fs::write(&output_file, png.as_bytes())?;
    } else {
        fs::write(&file_path, png.as_bytes())?;
    }

    Ok(())
}

pub fn decode(
    DecodeArgs {
        file_path,
        chunk_type,
    }: DecodeArgs,
) -> Result<()> {
    let png = read_png(&file_path)?;

    let chunk = png.chunk_by_type(&chunk_type.to_string());

    if let Some(chunk) = chunk {
        println!("Found message {}", chunk.data_as_string()?);
    } else {
        println!("No message found");
    }

    Ok(())
}

pub fn remove(
    RemoveArgs {
        file_path,
        chunk_type,
    }: RemoveArgs,
) -> Result<()> {
    let mut png = read_png(&file_path)?;
    png.remove_chunk(&chunk_type.to_string())?;
    fs::write(&file_path, png.as_bytes())?;
    Ok(())
}

pub fn print(PrintArgs { file_path }: PrintArgs) -> Result<()> {
    let png = read_png(&file_path)?;
    println!("{png}");
    Ok(())
}

fn read_png(file_path: &PathBuf) -> Result<Png> {
    let file = fs::read(&file_path)?;
    Png::try_from(file.as_slice())
}
