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
        compilation: External::new(Compilation {name: "owned by native".to_string()}),
      }
    }
    #[napi]
    pub fn create_compilation(&self) -> External<Compilation> {
      let external = External::new(Compilation{
        name: "owned by js".to_string()
      });
      
      external
    }
  }

  impl Drop for Compiler {
    fn drop(&mut self) {
      println!("Dropping Compiler");
    }
  }

  struct Compilation {
    name: String
  }
  impl Compilation {
    pub fn build(){
      println!("Building Compilation");
    }
  }
  impl Drop for Compilation {
    fn drop(&mut self) {
      println!("Dropping Compilation: {:?}",self.name);
    }
  }
}
