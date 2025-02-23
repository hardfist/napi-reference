use napi_derive::napi;
#[napi]
pub mod reference {
  use napi::bindgen_prelude::*;
  use napi_derive::napi;
  #[napi]
  pub struct Compiler {
    compilation: Reference<Compilation>,
  }

  #[napi]
  impl Compiler {
    #[napi(constructor)]
    pub fn new(compilation: Reference<Compilation>) -> Self {
      Compiler {
        compilation: compilation
      }
    }
  }

  impl Drop for Compiler {
    fn drop(&mut self) {
      println!("Dropping Compiler");
    }
  }
  #[napi]
  struct Compilation {}

  #[napi]
  impl Compilation {
    #[napi(constructor)]
    pub fn new() -> Self {
      Compilation {}
    }

  }

  impl Drop for Compilation {
    fn drop(&mut self) {
      println!("Dropping Compilation");
    }
  }
}
