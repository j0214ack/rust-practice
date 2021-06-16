use std::fmt;

#[derive(Debug)]
struct Complex {
  real: f32,
  imag: f32,
}

impl fmt::Display for Complex {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{0:.1} + {1:.1}i", self.real, self.imag)
  }
}


fn main() {
  let complex_number = Complex {
    real: 3.3,
    imag: 7.2,
  };

  println!("Display: {}", complex_number);
  println!("Display: {:?}", complex_number);
}
