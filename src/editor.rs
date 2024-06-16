use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::{assets, widgets::*};
use nih_plug_vizia::{create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::Arc;

use crate::OneXOscParams;

#[derive(Lens)]
struct AppData {
    params: Arc<OneXOscParams>,
}

impl Model for AppData {}

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (350, 200))
}

pub(crate) fn create(
    params: Arc<OneXOscParams>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);

        AppData {
            params: params.clone(),
        }
        .build(cx);

        Label::new(cx, "1xOSC");

        ResizeHandle::new(cx);
    })
}
