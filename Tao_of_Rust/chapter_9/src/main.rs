fn main() {
  {
    options_learning();
  }
}

fn options_learning() {
  fn get_shortest(names: Vec<&str>) -> Option<&str> {
    if names.len() > 0 {
      let mut shortest = names[0];
      for name in names.iter() {
        if name.len() < shortest.len() {
          shortest = *name;
        }
      }
      Some(shortest)
    } else {
      None
    }
  }
  {
    fn show_shortest(names: Vec<&str>) -> &str {
      match get_shortest(names) {
        Some(shortest) => shortest,
        None => "Not Found",
      }
    }

    assert_eq!(show_shortest(vec!["Uku", "Felipe"]), "Uku");
    assert_eq!(show_shortest(Vec::new()), "Not Found");
  }
  {
    fn show_shortest(names: Vec<&str>) -> &str {
      get_shortest(names).unwrap_or("Not Found")
      // get_shortest(names).unwrap_or_else( || "Not Found")
      // this will panicked and out put log
      // get_shortest(names).expect("Not Found")
    }

    assert_eq!(show_shortest(vec!["Uku", "Felipe"]), "Uku");
    assert_eq!(show_shortest(Vec::new()), "Not Found");
  }
  {
    fn get_shortest_length(names: Vec<&str>) -> Option<usize> {
      match get_shortest(names) {
        Some(shortest) => Some(shortest.len()),
        None => None,
      }
    }
    assert_eq!(get_shortest_length(vec!["Uku", "Felipe"]), Some(3));
    assert_eq!(get_shortest_length(Vec::new()), None);
  }
  {
    fn get_shortest_length(vec: Vec<&str>) -> Option<usize> {
      get_shortest(vec).map(|name| name.len())
    }
    assert_eq!(get_shortest_length(vec!["Uku", "Felipe"]), Some(3));
    assert_eq!(get_shortest_length(Vec::new()), None);
  }
  {
    fn double(value: f64) -> f64 {
      value * 2.0
    }
    fn square(value: f64) -> f64 {
      value.powi(2 as i32)
    }
    fn inverse(value: f64) -> f64 {
      value * -1.
    }
    fn log(value: f64) -> Option<f64> {
      match value.log2() {
        x if x.is_normal() => Some(x),
        _ => None,
      }
    }
    fn sqrt(value: f64) -> Option<f64> {
      match value.sqrt() {
        x if x.is_normal() => Some(x),
        _ => None,
      }
    }

    let number: f64 = 20.0;
    let result = Option::from(number)
      .map(inverse)
      .map(double)
      .map(inverse)
      .and_then(log)
      .map(square)
      .and_then(sqrt);
    match result {
      Some(x) => assert_eq!(x, 5.321928094887363),
      None => panic!("error"),
    }
  }
}
