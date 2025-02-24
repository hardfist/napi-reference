use napi_derive::napi;

#[napi]
pub mod tsfn {
    use std::{sync::Arc, thread};
    use napi::bindgen_prelude::{ObjectFinalize, *};
    use napi::threadsafe_function::ThreadsafeFunction;
    #[napi(custom_finalize)]
    pub struct Counter {
        tsfn: Arc<ThreadsafeFunction<i32>>,
        cnt: i32
    }
    #[napi]
    impl Counter{
        #[napi(constructor)]
        pub fn new(callback: Arc<ThreadsafeFunction<i32>>) -> Self {
            Self {
                tsfn:callback,
                cnt: 0
            }
        }
        #[napi]
        pub fn add(&mut self,env:Env,val: i32) -> napi::Result<()>{
            let tsfn = self.tsfn.clone();
            
            self.cnt +=val;
            let cnt = self.cnt.clone();
            println!("before call");
            tsfn.call(Ok(cnt), napi::threadsafe_function::ThreadsafeFunctionCallMode::NonBlocking);
            println!("after call");
            Ok(())
        }
    }
    impl Drop for Counter {
        fn drop(&mut self) {
           println!("drop");
        }
        
    }
    impl ObjectFinalize for Counter {
        fn finalize(mut self, env: Env) -> Result<()> {
            println!("finalization");
            // let tsfn = Arc::get_mut(&mut self.tsfn).unwrap();
            // tsfn.unref(&env)?;
            Ok(())
        } 
    }
}