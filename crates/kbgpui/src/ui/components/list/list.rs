use crate::ui::prelude::*;
use crate::ui::{Color, Label, LabelCommon};

/// Message shown when a [`List`] has no children.
pub enum EmptyMessage {
    Text(SharedString),
    Element(AnyElement),
}

/// A vertical list of items (Zed `List`).
#[derive(IntoElement)]
pub struct List {
    empty_message: EmptyMessage,
    children: Vec<AnyElement>,
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

impl List {
    pub fn new() -> Self {
        Self {
            empty_message: EmptyMessage::Text("No items".into()),
            children: Vec::new(),
        }
    }

    pub fn empty_message(mut self, message: impl Into<EmptyMessage>) -> Self {
        self.empty_message = message.into();
        self
    }
}

impl ParentElement for List {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl From<String> for EmptyMessage {
    fn from(s: String) -> Self {
        EmptyMessage::Text(SharedString::from(s))
    }
}

impl From<&str> for EmptyMessage {
    fn from(s: &str) -> Self {
        EmptyMessage::Text(SharedString::from(s.to_owned()))
    }
}

impl From<AnyElement> for EmptyMessage {
    fn from(e: AnyElement) -> Self {
        EmptyMessage::Element(e)
    }
}

impl RenderOnce for List {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        v_flex()
            .w_full()
            .py(DynamicSpacing::Base04.rems(cx))
            .map(|this| match self.children.is_empty() {
                false => this.children(self.children),
                true => match self.empty_message {
                    EmptyMessage::Text(text) => {
                        this.px_2().child(Label::new(text).color(Color::Muted))
                    }
                    EmptyMessage::Element(element) => this.child(element),
                },
            })
    }
}
