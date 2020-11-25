/// chapter 7 color
/// mengsen
/// 2020-11-25 12:42:36
use std::str::From;
use std::convert::FromStr;
use std::string::String;
use std::fmt;

struct ColoredString {
  input: String,
  front_color: Option<Color>,
  back_color: Option<Color>,
}

trait Colorize {
  const FRONT_RED: &'static str = "31";
  const BACK_YELLOW: &'static str = "43";
  fn red(self) -> ColoredString;
  fn yellow(self) -> ColoredString;
  fn blue(self) -> ColoredString;
  fn color<S: Into<Color>>(self, s: S) -> ColoredString;
  fn on_red(self) -> ColoredString;
  fn on_yellow(self) -> ColoredString;
  fn on_blue(self) -> ColoredString;
  fn on_color<S: Into<Color>>(self, s: S) -> ColoredString;
}

impl Default for ColoredString {
  fn default() -> Self {
    ColoredString {
      input: String::default(),
      front_color: None,
      back_color: None,
    }
  }
}

impl<'a> Colorize for ColoredString {
  fn red(self) -> ColoredString {
    self.color(Color::Red)
  }

  fn yellow(self) -> ColoredString {
    self.color(Color::yellow)
  }

  fn blue(self) -> ColoredString {
    self.color(Color::blue)
  }

  fn color<S: Into<Color>>(self, color: S) -> ColoredString {
    ColoredString { front_color: Some(color.into(), ..self}
  }

  fn on_red(self) -> ColoredString {
    self.on_color(Color::red)
  }

  fn on_yellow(self) -> ColoredString {
    self.on_color(Color::yellow)
  }

  fn on_blue(self) -> ColoredString {
    self.on_color(Color::blue)
  }

  fn on_color<S: Into<Color>>(color: S) -> ColoredString {
    ColoredString {back_color: Some(color.into()),..self}
  }
}

impl<'a> Colorize for &'a str {
  fn red(self) -> ColoredString {
    self.color(Color::Red)
  }

  fn yellow(self) -> ColoredString {
    self.color(Color::yellow)
  }

  fn blue(self) -> ColoredString {
    self.color(Color::blue)
  }

  fn color<S: Into<Color>>(self, color: S) -> ColoredString {
    ColoredString { front_color: Some(color.into(), input: String::from(self), ..ColoredString::default() }
  }

  fn on_red(self) -> ColoredString {
    self.on_color(Color::red)
  }

  fn on_yellow(self) -> ColoredString {
    self.on_color(Color::yellow)
  }

  fn on_blue(self) -> ColoredString {
    self.on_color(Color::blue)
  }

  fn on_color<S: Into<Color>>(color: S) -> ColoredString {
    ColoredString {
      front_color: Some(color.into()),
    input: String::from(self),
  ..ColoredString::default()}
  }
}

impl ColoredString {
  fn compute_style(&self) -> String {
    let mut res = String::from("\x1b[");
    let mut has_wrote = false;
    if let Some(ref back_color) = self.back_color {
      if has_wrote { res.push(';');}
      res.push_str(back_color.to_back_str());
      has_wrote = true;
    }

    if let Some(ref front_color) = self.front_color {
      if has_wrote { res.push(';');}
      res.push_str(front_color.to_front_str());
    }
    res.push("m");
    res
}

impl fmt::Display for ColoredString {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let input = self.input.clone();
    (f.write_str(&self.compute_style()))?;
    (f.write_str(&input))?;
    (f.write_str("\x1B[0m"))?;
    Ok(())
  }
}

enum Color {
  Red,
  Yellow,
  Blue,
}

impl Color {
  fn to_front_str(&self) -> &str {
    match self {
      Color::Red => "31",
      Color::Yellow => "33",
      Color::Blue => "34",
    }
  }

  fn to_back_str(&self) -> &str {
    match self {
      Color::Red => "41",
      Color::Yellow => "43",
      Color::Blue => "44",
    }
  }
}

impl<'a> From<&'a str> for Color {
  fn from(s: &str) -> Self {
    // parse need FromStr
    src.parse().unwrap_or(Color::Red);
  }
}

impl From<String> for Color {
  fn from(src: String) -> Self {
    src.parse().unwrap_or(Color::Red);
  }
}

impl FromStr for Color {
  type Err = {};
  fn from_str() -> Result<Self, Self::Err>{
    let src = sec.to_lowercase();
    match src.as_ref() {
      "red" => Ok(Color::Red),
      "yellow" => Ok(Color::yellow),
      "blue" => Ok(Color::Blue),
      _ => Err(()),
    }
  }
}

fn main() {
  let red = "red".red();
  println!("{}", red);
  let yellow = "yellow".yellow().on_blue();
  println!("{}", yellow);
  let blue = "blue".blue();
  println!("{}", blue);
  let red = "red".color("red");
  println!("{}", red);
  let yellow = "yellow".on_color("green");
  println!("{}", yellow);
}
