use napi_derive::napi;

#[napi]
pub mod tsfn {
  use napi::bindgen_prelude::{ObjectFinalize, *};
  use napi::threadsafe_function::ThreadsafeFunction;
  use std::{sync::Arc, thread};
  #[napi(custom_finalize)]
  pub struct Counter {
    tsfn: Arc<ThreadsafeFunction<i32, (), i32, false, false, 0>>,
    cnt: i32,
  }
  #[napi]
  impl Counter {
    #[napi(constructor)]
    pub fn new(callback: Arc<ThreadsafeFunction<i32, (), i32, false, false, 0>>) -> Self {
      Self {
        tsfn: callback,
        cnt: 0,
      }
    }
    #[napi]
    pub fn add(&mut self, env: Env, val: i32) -> napi::Result<()> {
      let tsfn = self.tsfn.clone();
      self.cnt += val;
      let cnt = self.cnt.clone();
      thread::spawn(move || {
        tsfn.call(
          cnt,
          napi::threadsafe_function::ThreadsafeFunctionCallMode::NonBlocking,
        );
      });
      Ok(())
    }
  }
  impl Drop for Counter {
    fn drop(&mut self) {
      println!("drop counter");
    }
  }
  impl ObjectFinalize for Counter {
    fn finalize(mut self, env: Env) -> Result<()> {
      println!("finalization");
      // let tsfn = Arc::get_mut(&mut self.tsfn).unwrap();
      // tsfn.unref(&env).unwrap();
      Ok(())
    }
  }
}
