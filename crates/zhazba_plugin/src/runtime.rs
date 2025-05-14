use std::{
  sync::{Arc, Mutex, mpsc},
  thread,
};

use anyhow::Result;
use tokio::{runtime::Builder, sync::oneshot};
use tracing::{debug, error};
use zhazba_lua::with_global_lua;


#[derive(Debug)]
pub struct Runtime {
  tx: mpsc::Sender<Task>,
  rx: Arc<Mutex<mpsc::Receiver<Task>>>,
}
impl Runtime {
  pub fn new() -> Self {
    let (sender, receiver) = mpsc::channel::<Task>();


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
            TaskKind::LoadModule => {
              match with_global_lua(|lua| lua.load(task.code).exec()) {
                Ok(_) => {}
                Err(err) => {
                  error!("{}", err);
                  panic!();
                }
              };
              task.responder.send(Ok(())).unwrap();
            }
            TaskKind::Execute => {}
          };
        });
      }
    });

    debug!("Plugin runtime started");
    return Ok(());
  }
  pub async fn load_module(&mut self, code: &str) -> Result<()> {
    let (responder, rx) = oneshot::channel::<Result<()>>();
    self.tx.send(Task {
      responder,
      code: code.to_string(),
      kind: TaskKind::LoadModule,
    })?;


    return rx.await?;
  }
}


struct Task {
  responder: oneshot::Sender<Result<()>>,
  code: String,
  kind: TaskKind,
}

enum TaskKind {
  LoadModule,
  Execute,
}
