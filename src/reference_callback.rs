use napi_derive::napi;
#[napi]
pub mod reference_callback {
    
    use napi::{bindgen_prelude::{ObjectFinalize, *}, JsFunction, Ref};


  #[napi(custom_finalize)]
  pub struct Compiler {
    callback:FunctionRef<(),()>
  }
  #[napi]
  impl Compiler {
    #[napi(constructor)]
    pub fn new(env: Env,callback: Function<(),()>) -> Self {
      Compiler { callback:callback.create_ref().unwrap()}
    }
    #[napi]
    pub fn run(&self, env: Env) -> napi::Result<()>{
       self.call_from_native(env);
       Ok(())
    }
    pub fn call_from_native(&self, env: napi::Env) -> Result<()>{
        self.callback.borrow_back(&env).unwrap().call(());
      
        Ok(())         
    }
  }
  impl ObjectFinalize for Compiler {
    fn finalize(mut self,env: Env)  -> napi::Result<()>{
      println!("drop compiler");
      Ok(())
    }
  }
}
