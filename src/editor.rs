use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::{assets, widgets::*};
use nih_plug_vizia::{create_vizia_editor, ViziaState, ViziaTheming};
use osc_buttons::BtnGroup;
use std::sync::Arc;

use crate::OneXOscParams;

mod icons;
mod osc_buttons;

#[derive(Lens, Clone)]
struct AppData {
    params: Arc<OneXOscParams>,
}

impl Model for AppData {}

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (300, 100))
}

pub(crate) fn create(
    params: Arc<OneXOscParams>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);

        cx.add_stylesheet(include_style!("src/editor/theme.css"))
            .ok();

        AppData {
            params: params.clone(),
        }
        .build(cx);

        VStack::new(cx, |cx| {
            BtnGroup::new(
                cx,
                AppData::params,
                |p| &p.osc_type,
                params.osc_type.value(),
            );
        })
        .class("app");

        ResizeHandle::new(cx);
    })
}
