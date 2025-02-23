use napi_derive::napi;
#[napi]
pub mod value {
  use napi::bindgen_prelude::*;
  use napi_derive::napi;
  #[napi(object)]
  struct Counter {
    pub cnt: i32
  }
  #[napi]
  pub fn create_counter(init: i32) -> Counter{
    Counter {
      cnt:init
    }
  }
}
