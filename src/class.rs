use napi_derive::napi;
#[napi]
pub mod _class {
  use napi::bindgen_prelude::*;
  #[napi]
  struct Counter {
    pub cnt: i32,
  }
  #[napi]
  impl Counter {
    #[napi(constructor)]
    pub fn new(init: i32) -> Self {
      Counter { cnt: init }
    }
    #[napi]
    pub fn add(&mut self, by: i32) -> i32 {
      self.cnt += by;
      self.cnt
    }
    #[napi]
    pub fn get(&self) -> i32 {
      self.cnt
    }
  }
}
