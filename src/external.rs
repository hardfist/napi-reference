use napi_derive::napi;
#[napi]
pub mod external {
  use napi::bindgen_prelude::*;
  use napi_derive::napi;
  #[napi]
  pub struct Compiler {
    compilation: External<Compilation>,
  }

  #[napi]
  impl Compiler {
    #[napi(constructor)]
    pub fn new() -> Self {
      Compiler {
        compilation: External::new(Compilation {}),
      }
    }
  }

  impl Drop for Compiler {
    fn drop(&mut self) {
      println!("Dropping Compiler");
    }
  }

  struct Compilation {}

  impl Drop for Compilation {
    fn drop(&mut self) {
      println!("Dropping Compilation");
    }
  }
}
