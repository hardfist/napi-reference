use napi_derive::napi;
#[napi]
pub mod deadlock {
  use std::time::Duration;

  use napi::bindgen_prelude::spawn;
  use tokio::{runtime::Handle, time::sleep};

  #[napi]
  pub struct DeadLocker {}
  #[napi]
  impl DeadLocker {
    #[napi(constructor)]
    pub fn new() -> Self {
      DeadLocker {}
    }
    // deadlock caused by block_in_place
    #[napi]
    pub async fn bip(&self) -> napi::Result<()> {
      (1..100).into_iter().for_each(|_| {
        tokio::task::block_in_place(|| {
          tokio::runtime::Builder::new_current_thread().enable_all()
            .build()
            .unwrap()
            .block_on(async {
              println!("block_in_place");
              sleep(Duration::from_secs(100)).await
            });
        });
      });
      (1..100).into_iter().for_each(|_| {
        spawn(async move {
          println!("spawn job");
        });
      });

      Ok(())
    }
  }
}
