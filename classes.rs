use std::io;

fn main() {
  let filename = "testing.txt";
  let text = "This is a testing test!";

  let f = FileIO::new(filename);
  println!("Writing to file {}...", filename);
  f.write(text);
  println!("Wrote: {}", text);
  println!("Reading from file {}...", filename);
  let res: &str = f.read();
  println!("Read: {}", res);
}

struct FileIO {
  path: Path,
}

impl FileIO {
  fn new(filename: &str) -> FileIO {
    FileIO {
      path: Path::new(filename)
    }
  }

  fn write(&self, text: &str) {
    let mut file = match io::File::open_mode(&self.path, io::Open, io::Write) {
      Ok(f) => f,
      Err(e) => fail!("Can't open file for Write {}: {}", self.path.filename(), e)
    };
    match file.write_str(text) {
      Ok(()) => (),
      Err(e) => fail!("Can't write to file {}: {}", self.path.filename(), e)
    };
  }

  fn read(&self) -> (~str) {
    let mut file = match io::File::open_mode(&self.path, io::Open, io::Read) {
      Ok(f) => f,
      Err(e) => fail!("Can't open file for Read {}: {}", self.path.filename(), e)
    };
    match file.read_to_str() {
      Ok(s) => s,
      Err(e) => fail!("Can't read file {}: {}", self.path.filename(), e)
    }
  }
}
