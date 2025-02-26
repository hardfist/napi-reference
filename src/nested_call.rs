use napi_derive::napi;

#[napi]
pub mod nested_call {

    use std::sync::Arc;

    use napi::{bindgen_prelude::*, threadsafe_function::ThreadsafeFunction};


    #[napi(custom_finalize)]
    pub struct Compiler {
        asset_path_callback: FunctionRef<i32,i32>,
        tsfn:Arc<ThreadsafeFunction<i32, i32, i32, false>>
    }
    #[napi]
    impl Compiler {
        #[napi(constructor)]
        pub fn new(env: Env,callback: Function<i32,i32>) -> Self {
            let mut tsfn: napi::threadsafe_function::ThreadsafeFunction<i32, i32, i32, false> = callback.build_threadsafe_function().build().unwrap();
            tsfn.unref(&env).unwrap();
            Compiler {
                asset_path_callback:callback.create_ref().unwrap(),
                tsfn: Arc::new(tsfn)
            }
        }
        #[napi]
        pub fn get_asset_path(&self,env: Env,n:i32) -> i32{
            self.call_assset_path_hook(env,n)
        }
        #[napi]
        pub fn call_assset_path_hook(&self,env: Env,n:i32) -> i32{  
             // this will cause dead lock
             // let result = block_on(self.tsfn.call_async(100)).unwrap(); 
             let result = self.asset_path_callback.borrow_back(&env).unwrap().call(n).unwrap();
             result
        }
    }
    
    impl ObjectFinalize for Compiler {
        fn finalize(self, _env: Env) -> napi::Result<()> {
            
            println!("drop compiler");
            Ok(())
        }
    }
}