use std::io;

fn main() {
  let filename = "testing.txt";
  let text = "This is a testing test!";
  println!("Writing to file {}...", filename);
  write(filename, text);
  println!("Wrote: {}", text);
  println!("Reading from file {}...", filename);
  let res: &str = read(filename);
  println!("Read: {}", res);
}

fn write(filename: &str, text: &str) {
  let p = Path::new(filename);

  let mut file = match io::File::open_mode(&p, io::Open, io::Write) {
    Ok(f) => f,
    Err(e) => fail!("Can't open file for Write {}: {}", filename, e)
  };
  match file.write_str(text) {
    Ok(()) => (),
    Err(e) => fail!("Can't write to file {}: {}", filename, e)
  };
}

fn read(filename: &str) -> (~str) {
  let p = Path::new(filename);

  let mut file = match io::File::open_mode(&p, io::Open, io::Read) {
    Ok(f) => f,
    Err(e) => fail!("Can't open file for Read {}: {}", filename, e)
  };
  match file.read_to_str() {
    Ok(s) => s,
    Err(e) => fail!("Can't read file {}: {}", filename, e)
  }
}
