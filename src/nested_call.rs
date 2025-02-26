use napi_derive::napi;

#[napi]
pub mod nested_call {

    use napi::bindgen_prelude::{Function, FunctionRef, Env};


    #[napi]
    pub struct Compiler {
        pub asset_path_callback: FunctionRef<i32,i32>
    }
    #[napi]
    impl Compiler {
        #[napi(constructor)]
        pub fn new(callback: Function<i32,i32>) -> Self {
            Compiler {
                asset_path_callback:callback.create_ref().unwrap()
            }
        }
        #[napi]
        pub fn get_asset_path(&self,env: Env,n:i32) -> i32{
            self.call_assset_path_hook(env,n)
        }
        #[napi]
        pub fn call_assset_path_hook(&self,env: Env,n:i32) -> i32{
             let result = self.asset_path_callback.borrow_back(&env).unwrap().call(n).unwrap();
             result
        }
    }
    
}