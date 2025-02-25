use napi_derive::napi;
#[napi]
pub mod value {

  use napi::{bindgen_prelude::*, JsObject};
  use serde_derive::{Deserialize, Serialize};

  #[derive(Serialize, Deserialize)]
  #[napi(object)]
  struct Counter {
    pub cnt: i32,
  }
  #[napi]
  pub fn create_counter(init: i32) -> Counter {
    Counter { cnt: init }
  }
  #[napi]
  pub fn from_js_clone(env: Env, counter: Counter) -> Counter {
    counter
  }
  #[napi]
  pub fn from_js_serde(env: Env, counter: JsObject) -> Counter {
    let a: Counter = env.from_js_value(counter).unwrap();
    a
  }
}
