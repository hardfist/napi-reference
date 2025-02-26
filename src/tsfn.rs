use napi_derive::napi;

#[napi]
pub mod tsfn {
  use napi::bindgen_prelude::{ObjectFinalize, *};
  use napi::threadsafe_function::ThreadsafeFunction;
  use std::thread::sleep;
use std::time::Duration;
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
    pub fn add(&mut self, _env: Env, val: i32) -> napi::Result<()> {
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
    fn finalize(self, _env: Env) -> Result<()> {
      println!("finalization");
      // let tsfn = Arc::get_mut(&mut self.tsfn).unwrap();
      // tsfn.unref(&env).unwrap();
      Ok(())
    }
  }

   #[napi]
  pub struct Container {
    tsfn: Arc<ThreadsafeFunction<i32, (), i32, false, true, 0>>,
  }
  #[napi]
  impl Container {
    #[napi(constructor)]
    pub fn new(callback: Arc<ThreadsafeFunction<i32, (), i32, false, true, 0>>) -> Self {
      Self {
        tsfn: callback,
      }
    }
    #[napi]
    pub fn run_in_background(&mut self, _env: Env, val: i32) -> napi::Result<()> {
      let tsfn = self.tsfn.clone();
      thread::spawn(move || {
        sleep(Duration::from_secs(2));
        tsfn.call(
          val,
          napi::threadsafe_function::ThreadsafeFunctionCallMode::NonBlocking,
        );
      });
      Ok(())
    }
  }
}
