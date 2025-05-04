use std::{
  sync::{Arc, Mutex, mpsc},
  thread,
};

use anyhow::Result;
use tokio::{runtime::Builder, sync::oneshot};


#[derive(Debug)]
pub struct Runtime {
  tx: mpsc::SyncSender<Task>,
  rx: Arc<Mutex<mpsc::Receiver<Task>>>,
}
impl Runtime {
  pub fn new() -> Self {
    let (sender, receiver) = mpsc::sync_channel::<Task>(0);

    return Self {
      tx: sender,
      rx: Arc::new(Mutex::new(receiver)),
    };
  }

  pub fn init(&self) -> Result<()> {
    let rx = self.rx.clone();

    thread::spawn(move || {
      let rt = Builder::new_current_thread().enable_all().build().unwrap();

      for task in rx.lock().unwrap().iter() {
        rt.block_on(async {
          match task.kind {
            TaskKind::Module => {}
            TaskKind::Execute => {}
          };
        });
      }
    });

    return Ok(());
  }
}
unsafe impl Sync for Runtime {}


struct Task {
  responder: oneshot::Sender<Result<()>>,
  code: String,
  kind: TaskKind,
}

enum TaskKind {
  Module,
  Execute,
}
