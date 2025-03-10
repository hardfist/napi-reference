use napi_derive::napi;
#[napi]
pub mod async_fn {
  use napi::threadsafe_function::ThreadsafeFunction;
  use std::{rc::Rc, sync::Arc, thread, time::Duration};
  #[napi]
  async fn async_fn() -> napi::Result<()> {
    // let message = Rc::new("hello"); remove this will cause compile error
    tokio::time::sleep(Duration::from_millis(100)).await;
    // println!("message: {}",message);
    Ok(())
  }
  #[napi]
  async fn async_fn1() -> napi::Result<()> {
    let _future = async move {
      let message = Rc::new("hello");
      tokio::time::sleep(Duration::from_millis(100)).await;
      println!("message: {}", message);
    };

    // tokio::spawn(future).await.unwrap();

    Ok(())
  }
  #[napi]
  pub fn async_fn2(
    callback: Arc<ThreadsafeFunction<String, (), String, false, false, 0>>,
  ) -> napi::Result<()> {
    thread::spawn(move || {
      let future = async move {
        let message = Rc::new("hello");
        tokio::time::sleep(Duration::from_millis(100)).await;
        println!("message: {}", message);
        message.to_string()
      };
      let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
      let result = rt.block_on(future);
      callback.call(
        result,
        napi::threadsafe_function::ThreadsafeFunctionCallMode::NonBlocking,
      );
    });
    Ok(())
  }
}
