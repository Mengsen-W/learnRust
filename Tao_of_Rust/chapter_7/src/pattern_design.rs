/// Builder Pattern
/// mengsen
/// 2020-11-26 17:49:41

struct Circle {
  x: f64,
  y: f64,
  radius: f64,
}

struct CircleBuilder {
  x: f64,
  y: f64,
  radius: f64,
}

impl Circle {
  fn area(&self) -> f64 {
    std::f64::consts::PI * (self.radius * self.radius)
  }

  fn new() -> CircleBuilder {
    CircleBuilder {
      x: 0.0,
      y: 0.0,
      radius: 1.0,
    }
  }
}

impl CircleBuilder {
  fn x(&mut self, coordinate: f64) -> &mut CircleBuilder {
    self.x = coordinate;
    self
  }

  fn y(&mut self, coordinate: f64) -> &mut CircleBuilder {
    self.y = coordinate;
    self
  }

  fn radius(&mut self, coordinate: f64) -> &mut CircleBuilder {
    self.radius = coordinate;
    self
  }

  fn build(&self) -> Circle {
    Circle {
      x: self.x,
      y: self.y,
      radius: self.radius,
    }
  }
}

#[derive(Debug)]
pub struct Letter {
  text: String,
}

pub struct EmptyEnvelope {}
pub struct ClosedEnvelope {
  letter: Letter,
}
pub struct PickupLorryHandle {
  done: bool,
}

impl Letter {
  pub fn new(text: String) -> Self {
    Letter { text: text }
  }
}

impl EmptyEnvelope {
  pub fn wrap(self, letter: Letter) -> ClosedEnvelope {
    ClosedEnvelope { letter: letter }
  }
}

pub fn buy_prestamped_envelope() -> EmptyEnvelope {
  EmptyEnvelope {}
}

impl PickupLorryHandle {
  pub fn pickup(&mut self, envelope: ClosedEnvelope) {
    /* give letter */
  }
  pub fn done(self) {}
}

impl Drop for PickupLorryHandle {
  fn drop(&mut self) {
    println!("send");
  }
}

pub fn order_pickup() -> PickupLorryHandle {
  PickupLorryHandle {
    done: false, /* other handles */
  }
}

fn main() {
  {
    // builder pattern
    let c = Circle::new().x(1.0).y(2.0).radius(2.0).build();
    assert_eq!(12.566370614359172, c.area());
  }

  {
    let letter = Letter::new(String::from("Dear Rust"));
    let mut envelope = buy_prestamped_envelope();
    let closed_envelope = envelope.wrap(letter);
    let mut lorry = order_pickup();
    lorry.pickup(closed_envelope);
  }
}
