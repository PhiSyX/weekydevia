use std::ffi::OsString;

use tokio::sync::mpsc;

// ---- //
// Type //
// ---- //

pub(crate) type TemplateReceiver = mpsc::UnboundedReceiver<TemplateState>;
pub(crate) type TemplateSender = mpsc::UnboundedSender<TemplateState>;

// --------- //
// Structure //
// --------- //

pub struct TemplateChan(pub TemplateSender, pub TemplateReceiver);

// ----------- //
// Énumération //
// ----------- //

pub enum TemplateState {
    Content { filename: OsString, text: String },
    EOF,
}

// -------------- //
// Implémentation //
// -------------- //

impl TemplateChan {
    pub fn channel() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        Self(tx, rx)
    }
}
