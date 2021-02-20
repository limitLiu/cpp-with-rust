#[no_mangle]
pub extern "C" fn my_sqrt(x: f64) -> f64 {
  sqrt_iter(1.0, x)
}

fn good_enough(guess: f64, x: f64) -> bool {
  (square(guess) - x).abs() < 0.001
}

fn average(x: f64, y: f64) -> f64 {
  (x + y) / 2.0
}

fn improve(guess: f64, x: f64) -> f64 {
  average(guess, x / guess)
}

fn sqrt_iter(guess: f64, x: f64) -> f64 {
  if good_enough(guess, x) {
    guess
  } else {
    sqrt_iter(improve(guess, x), x)
  }
}

#[no_mangle]
pub extern "C" fn my_cbrt(x: f64) -> f64 {
  cbrt_iter(1.0, x)
}

fn cbrt_iter(y: f64, x: f64) -> f64 {
  if cbrt_good_enough(y, x) {
    y
  } else {
    cbrt_iter(cbrt_improve(y, x), x)
  }
}

fn cbrt_improve(y: f64, x: f64) -> f64 {
  (x / square(y) + 2.0 * y) / 3.0
}

fn cbrt_good_enough(y: f64, x: f64) -> bool {
  (cube(y) - x).abs() < 0.001
}

// tool functions

fn square(x: f64) -> f64 {
  x * x
}

fn cube(x: f64) -> f64 {
  x * x * x
}
