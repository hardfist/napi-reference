use napi_derive::napi;
#[napi]
pub mod reference_callback {
    
    use napi::{bindgen_prelude::{ObjectFinalize, *}, JsFunction, Ref};


  #[napi(custom_finalize)]
  pub struct Compiler {
    callback:Ref<()>
  }
  #[napi]
  impl Compiler {
    #[napi(constructor)]
    pub fn new(env: Env,callback: JsFunction) -> Self {
      Compiler { callback: env.create_reference(callback).unwrap()}
    }
    #[napi]
    pub fn run(&self, env: Env) -> napi::Result<()>{
       self.call_from_native(env);
       Ok(())
    }
    pub fn call_from_native(&self, env: napi::Env) -> Result<()>{
        let js_fn:JsFunction = env.get_reference_value(&self.callback)?;
        js_fn.call_without_args(None)?;
        Ok(())         
    }
  }
  impl ObjectFinalize for Compiler {
    fn finalize(mut self,env: Env)  -> napi::Result<()>{
      println!("drop compiler with unref");
      self.callback.unref(env).unwrap();
      Ok(())
    }
  }
}
