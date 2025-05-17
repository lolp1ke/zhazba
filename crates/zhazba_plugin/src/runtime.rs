use std::{sync::mpsc, thread};

use anyhow::Result;
use tokio::{runtime::Builder, sync::oneshot};
use tracing::{debug, error};

use zhazba_lua::with_global_lua;


#[derive(Debug)]
pub struct Runtime {
  tx: mpsc::Sender<Task>,
}
impl Runtime {
  pub fn new() -> Self {
    let (sender, receiver) = mpsc::channel::<Task>();
    thread::spawn(move || {
      let rt = Builder::new_current_thread().enable_all().build().unwrap();

      for task in receiver {
        rt.block_on(async move {
          match task.kind {
            TaskKind::LoadModule => {
              match with_global_lua(|lua| lua.load(task.code).exec()) {
                Ok(_) => task.responder.send(Ok(())).unwrap(),
                Err(_) => panic!(),
              };
            }
            TaskKind::Execute => {}
          };
        });
      }
    });


    debug!("PluginRuntime::new()");
    return Self { tx: sender };
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
