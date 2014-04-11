use std::io;


static base_path: &'static str = "";


enum Direction {
  OUT,
  IN
}

enum Value {
  LOW,
  HIGH
}


struct Pin {
  pin: int
}

impl Pin {
  pub fn new(p: int) -> Pin {
    Pin {
      pin: p
    }
  }

  fn get_path(&self, ep: &str) -> (~str) {
    //format!("{}/{}/{}", base_path, self.pin, ep)
    format!("{}/{}", self.pin, ep)
  }

  pub fn export(&self) {
    //let path = Path::new(format!("{}/export", base_path));
    let path = Path::new("export");
    let mut file = match io::File::open_mode(&path, io::Open, io::Write) {
      Ok(f) => f,
      Err(e) => fail!("Can't open export file: {}", e)
    };
    match file.write_int(self.pin) {
      Ok(()) => (),
      Err(e) => fail!("Can't write to export file: {}", e)
    };
    match file.fsync() {
      Ok(()) => (),
      Err(e) => fail!("Can't sync export file: {}", e)
    }
  }

  pub fn unexport(&self) {
    //let path = Path::new(format!("{}/unexport", base_path));
    let path = Path::new("unexport");
    let mut file = match io::File::open_mode(&path, io::Open, io::Write) {
      Ok(f) => f,
      Err(e) => fail!("Can't open unexport file: {}", e)
    };
    match file.write_int(self.pin) {
      Ok(()) => (),
      Err(e) => fail!("Can't write to unexport file: {}", e)
    };
    match file.fsync() {
      Ok(()) => (),
      Err(e) => fail!("Can't sync unexport file: {}", e)
    }
  }

  pub fn set_direction(&self, d: Direction) {
    let path = Path::new(self.get_path("direction"));
    let a = match d {
      OUT => "out",
      IN => "in"
    };
    let mut file = match io::File::open_mode(&path, io::Open, io::Write) {
      Ok(f) => f,
      Err(e) => fail!("Can't open pin direction file: {}", e)
    };
    match file.write_str(a) {
      Ok(()) => (),
      Err(e) => fail!("Can't write to pin direction file: {}", e)
    };
    match file.fsync() {
      Ok(()) => (),
      Err(e) => fail!("Can't sync pin direction file: {}", e)
    }
  }

  pub fn input(&self) {
    self.set_direction(IN);
  }

  pub fn output(&self) {
    self.set_direction(OUT);
  }

  pub fn get_direction(&self) -> (Direction) {
    let path = Path::new(self.get_path("direction"));
    let file = match io::File::open_mode(&path, io::Open, io::Read).read_to_str() {
      Ok(f) => f,
      Err(e) => fail!("Can't read pin direction: {}", e)
    };
    if file == ~"in" {
      IN
    } else {
      OUT
    }
  }

  pub fn set_value(&self, v: Value) {
    let path = Path::new(self.get_path("value"));
    let mut file = match io::File::open_mode(&path, io::Open, io::Write) {
      Ok(f) => f,
      Err(e) => fail!("Can't open pin value file: {}", e)
    };
    match file.write_int((v as int)) {
      Ok(()) => (),
      Err(e) => fail!("Can't write to pin value file: {}", e)
    };
    match file.fsync() {
      Ok(()) => (),
      Err(e) => fail!("Can't sync pin value file: {}", e)
    }
  }

  pub fn high(&self) {
    self.set_value(HIGH);
  }

  pub fn low(&self) {
    self.set_value(LOW);
  }

  pub fn get_value(&self) -> (Value) {
    let path = Path::new(self.get_path("value"));
    let i = match io::File::open_mode(&path, io::Open, io::Read).read_to_str() {
      Ok(f) => f,
      Err(e) => fail!("Can't read pin direction: {}", e)
    };
    if i == ~"0" {
      LOW
    } else {
      HIGH
    }
  }
}

#[test]
fn test_directions() {
  let f = Pin::new(4);

  f.set_direction(IN);
  assert!(f.get_direction(), 1);

  f.set_direction(OUT);
  assert!(f.get_direction(), 0);

  f.input();
  assert!(f.get_direction(), IN);

  f.output();
  assert!(f.get_direction(), OUT);
}

#[test]
fn test_values() {
  let f = Pin::new(4);

  f.set_value(HIGH);
  assert!(f.get_value(), HIGH);

  f.set_value(LOW);
  assert!(f.get_value(), LOW);

  f.high();
  assert!(f.get_value(), HIGH);

  f.low();
  assert!(f.get_value(), LOW);
}

fn main() {
  let f = Pin::new(4);

  f.export();

  f.set_direction(IN);
  match f.get_direction() {
    IN => println!("in"),
    OUT => println!("out")
  };

  f.set_direction(OUT);
  match f.get_direction() {
    IN => println!("in"),
    OUT => println!("out")
  };

  f.input();
  match f.get_direction() {
    IN => println!("in"),
    OUT => println!("out")
  };

  f.output();
  match f.get_direction() {
    IN => println!("in"),
    OUT => println!("out")
  };

  f.set_value(HIGH);
  match f.get_value() {
    HIGH => println!("high"),
    LOW => println!("low")
  };

  f.set_value(LOW);
  match f.get_value() {
    HIGH => println!("high"),
    LOW => println!("low")
  };

  f.high();
  match f.get_value() {
    HIGH => println!("high"),
    LOW => println!("low")
  };

  f.low();
  match f.get_value() {
    HIGH => println!("high"),
    LOW => println!("low")
  };

  f.unexport();
}
