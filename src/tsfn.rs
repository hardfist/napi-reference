use napi_derive::napi;

#[napi]
pub mod tsfn {
    use std::{sync::Arc, thread};

    use napi::threadsafe_function::ThreadsafeFunction;
    #[napi]
    pub struct Counter {
        tsfn: Arc<ThreadsafeFunction<i32>>,
        cnt: i32
    }
    #[napi]
    impl Counter{
        #[napi(constructor)]
        pub fn new(callback: ThreadsafeFunction<i32>) -> Self {
            Self {
                tsfn: Arc::new(callback),
                cnt: 0
            }
        }
        #[napi]
        pub fn add(&mut self,val: i32){
            let tsfn = self.tsfn.clone();
            self.cnt +=val;
            let cnt = self.cnt.clone();
            thread::spawn(move || {
                tsfn.call(Ok(cnt), napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking);
            });
        }
    }
    impl Drop for Counter {
        fn drop(&mut self) {
           println!("drop");
        }
        
    }
}