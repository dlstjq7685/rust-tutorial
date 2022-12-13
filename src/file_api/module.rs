use std::fs::{File, self};
use std::io::{BufRead, BufReader, Error, ErrorKind, Write, Read, self};
use std::path::Path;

pub fn call() {

    println!("call file_api module");
    
    let file_path = String::from("test.txt");

    if let Err(e) = read_file(&file_path) { 
        occpy_panic(&e) 
    }

}

fn read_file(file_path: &String)  -> Result<(), Error> {

    println!("{}", file_path);
    let path_to_read = Path::new(file_path);

    let file = File::open(path_to_read)?;

    let buffered = BufReader::new(file);

    for line in buffered.lines() {
        println!("{}", line?);
    }

    Ok(())
}

fn write_byte_file(file_path: &String) -> io::Result<()>{


    let path = Path::new(file_path);

    let mut buffer_modified = Vec::new();
    buffer_modified.push('H' as u8);
    buffer_modified.push('e' as u8);
    buffer_modified.push('l' as u8);
    buffer_modified.push('l' as u8);
    buffer_modified.push('o' as u8);
    buffer_modified.push('.' as u8);
        
    let mut f = File::create(path)?;
    f.write_all(buffer_modified.as_slice())?;

    Ok(())    
}


fn walk_dir() {

}

fn delete_file(file_path: &String) -> std::io::Result<()> {

    let path = Path::new(file_path);

    fs::remove_file(path)?;

    Ok(())

}

fn occpy_panic(err :&Error) {

    panic!("{}", err);

}