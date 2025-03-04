pub use async_trait::async_trait;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

/// Plain old JSON objects are what the transformations expect to receive and produce.
pub type JsObject = serde_json::Map<String, serde_json::Value>;
/// A channel for plugins to send payloads/scripts to for execution.
pub type TransformerTx = mpsc::UnboundedSender<TransformerJob>;
/// The receiver side for transformations. The JS executor reads from this.
pub type TransformerRx = mpsc::UnboundedReceiver<TransformerJob>;
/// A oneshot channel for the JS executor to "publish" return values to once complete.
// FIXME: better error type?
pub type TransformerCallbackTx = oneshot::Sender<Result<JsReturn, ()>>;
/// Used by the caller of the transformer to await the execution's output.
// FIXME: better error type?
pub type TransformerCallbackRx = oneshot::Receiver<Result<JsReturn, ()>>;

/// A transformation job sent to the JS executor.
/// Once the script has been run on the payload, the transformed payload is sent back through the
/// callback channel.
pub struct TransformerJob {
    pub callback_tx: TransformerCallbackTx,
    pub payload: JsObject,
    pub script: String,
}

pub enum JsReturn {
    /// A successfully transformed payload.
    // XXX: not sure if there's a cheaper way to deserialize the output while requiring an Object.
    Object(JsObject),
    /// For cases where the JS script executes successfully but produces an unexpected output.
    Invalid,
}

impl TransformerJob {
    pub fn new(script: String, payload: JsObject) -> (Self, TransformerCallbackRx) {
        let (callback_tx, callback_rx) = oneshot::channel();
        (
            Self {
                payload,
                script,
                callback_tx,
            },
            callback_rx,
        )
    }
}

/// Effectively a black box to the supervisor.
/// Plugins should run until they are done, and likely they should not be "done" until the program
/// exits.
#[async_trait]
pub trait Plugin: Send {
    /// For plugins that want to run JS transformations on payloads.
    /// Giving them a sender lets them pass messages to the JS executor.
    fn set_transformer(&mut self, _tx: Option<TransformerTx>) {}
    async fn run(&self) -> std::io::Result<()>;
}
