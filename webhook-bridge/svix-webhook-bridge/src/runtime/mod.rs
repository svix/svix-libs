use anyhow::Result;
use std::sync::Arc;

use deno_core::{anyhow, serde_v8, v8, JsRuntime};
use svix_webhook_bridge_types::{JsObject, JsReturn};
use threadpool::ThreadPool;
use tokio::sync::{oneshot, Mutex};

#[derive(Clone, Debug)]
pub struct TpHandle(pub Arc<Mutex<ThreadPool>>);

impl TpHandle {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(ThreadPool::default())))
    }

    pub async fn run_script(&self, input: serde_json::Value, script: String) -> Result<JsReturn> {
        let (tx, rx) = oneshot::channel();

        self.0.lock().await.execute(move || {
            let _ = tx.send(run_script_inner(&input, script));
        });

        rx.await?
    }
}

fn run_script_inner(input: &serde_json::Value, script: String) -> Result<JsReturn> {
    let mut runtime = JsRuntime::new(Default::default());
    let res = runtime.execute_script(
        "<anon>",
        &format!("{script}\nhandler({})", serde_json::to_string(input)?),
    );
    match res {
        Ok(global) => {
            let scope = &mut runtime.handle_scope();
            let local = v8::Local::new(scope, global);
            match serde_v8::from_v8::<JsObject>(scope, local) {
                Ok(v) => Ok(JsReturn::Object(v)),
                Err(serde_v8::Error::ExpectedObject) => Ok(JsReturn::Invalid),
                Err(e) => Err(e)?,
            }
        }
        Err(err) => Err(anyhow::format_err!("Evaling error: {:?}", err)),
    }
}

#[cfg(test)]
mod tests;
