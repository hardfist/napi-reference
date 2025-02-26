use napi_derive::napi;

#[napi]
pub mod external_value {
  use napi::bindgen_prelude::*;
  
  #[napi(object)]
  struct Counter {
    pub cnt: i32,
  }

  #[napi]
  pub fn create_counter(init: i32) -> External<Counter> {
    External::new(Counter { cnt: init })
  }
  #[napi]
  pub fn add_counter( counter: &mut External<Counter>, by: i32) -> i32 {
    counter.cnt += by;
    counter.cnt
  }
  #[napi]
  pub fn print_external(counter: &External<Counter>) {
    println!("Counter: {}", counter.cnt);
  }
}
