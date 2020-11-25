/// chapter 7 color
/// mengsen
/// 2020-11-25 12:42:36

use std::fmt;

struct ColoredString {
  input: String,
  front_color: String,
  back_color: String,
}

trait Colorize {
  const Front_RED: &'static str = "31";
  const Back_YELLOW: &'static str = "43";
  fn red(self) -> ColoredString;
  fn on_yellow(self) -> ColoredString;
}

impl Default for ColoredString {
  fn default() -> Self {
    ColoredString {
      input: String::default(),
      front_color: String::default(),
      back_color: String::default(),
    }
  }
}
impl<'a> Colorize for ColoredString {
  fn red(self) -> ColoredString {
    ColoredString {
      front_color: String::from(ColoredString::Front_RED),
      ..self
    }
  }
  fn on_yellow(self) -> ColoredString {
    ColoredString {
      back_color: String::from(ColoredString::Back_YELLOW),
      ..self
    }
  }
}

impl<'a> Colorize for &'a str {
  fn red(self) -> ColoredString {
    ColoredString {
      front_color: String::from(ColoredString::Front_RED),
      input: String::from(self),
      ..ColoredString::default()
    }
  }
  fn on_yellow(self) -> ColoredString {
    ColoredString {
      back_color: String::from(ColoredString::Back_YELLOW),
      input: String::from(self),
      ..ColoredString::default()
    }
  }
}

impl ColoredString {
  fn compute_style(&self) -> String {
    let mut res = String::from("\x1b[");
    let mut has_wrote = false;
    if !self.back_color.is_empty() {
      res.push_str(self.back_color)
      has_wrote = true;
    }
    if !self.front_color.is_empty() {
      if has_wrote {res.push(';');}
      res.push_str(&self.front_color);
    }
    res.push('m');
    res
  }
}

impl fmt::Display for ColoredString {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
    let mut input = &self.input.clone();
    try!(f.write_str(&self.compute_style()));
    try!(f.write_str(input));
    try!(f.write_str("\x1B[0m"));
    Ok(())
  }
}