use std::io;


fn main() {
  let f = Pin::new(46);
  f.set_direction(IN);
  let res = f.get_direction();
  println!("Direction: {}", (res as int));
}

static base_path: &'static str = "";

enum Direction {
  IN,
  OUT
}

enum Value {
  LOW,
  HIGH
}


struct Pin {
  pin: int,
}

impl Pin {
  fn new(p: int) -> Pin {
    Pin {
      pin: p
    }
  }

  fn export(&self) {
    let path = Path::new(format!("{}/{}/export", base_path, self.pin));
    let mut file = match io::File::open_mode(&path, io::Open, io::Write).write_int(self.pin) {
      Ok(f) => f,
      Err(e) => fail!("Can't export pin: {}", e)
    };
  }

  fn unexport(&self) {
    let path = Path::new(format!("{}/{}/unexport", base_path, self.pin));
    let mut file = match io::File::open_mode(&path, io::Open, io::Write).write_int(self.pin) {
      Ok(f) => f,
      Err(e) => fail!("Can't export pin: {}", e)
    };
  }

  fn set_direction(&self, d: Direction) {
    let path = Path::new(format!("{}/{}/direction", base_path, self.pin));
    let a = match d {
      OUT => "out",
      IN => "in"
    };
    let mut file = match io::File::open_mode(&path, io::Open, io::Write).write_str(a) {
      Ok(()) => (),
      Err(e) => fail!("Can't set pin direction: {}", e)
    };
  }

  fn input(&self) {
    self.set_direction(IN);
  }

  fn output(&self) {
    self.set_direction(OUT);
  }

  fn get_direction(&self) -> (Direction) {
    let path = Path::new(format!("{}/{}/direction", base_path, self.pin));
    let mut file = match io::File::open_mode(&path, io::Open, io::Read).read_to_str() {
      Ok(f) => f,
      Err(e) => fail!("Can't read pin direction: {}", e)
    };
    if file == ~"in" {
      IN
    } else {
      OUT
    }
  }

  fn set_value(&self, v: Value) {
    let path = Path::new(format!("{}/{}/value", base_path, self.pin));
    let mut file = match io::File::open_mode(&path, io::Open, io::Write).write_int((v as int)) {
      Ok(()) => (),
      Err(e) => fail!("Can't set pin value: {}", e)
    };
  }
}
