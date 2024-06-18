use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::{assets, widgets::*};
use nih_plug_vizia::{create_vizia_editor, ViziaState, ViziaTheming};
use osc_buttons::BtnGroup;
use std::sync::Arc;

use crate::OneXOscParams;

mod osc_buttons;

#[derive(Lens, Clone)]
struct AppData {
    params: Arc<OneXOscParams>,
}

impl Model for AppData {}

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (300, 100))
}

macro_rules! load_image {
    ($cx: expr,$path1:expr, $path2:expr) => {
        $cx.load_image(
            $path1,
            image::load_from_memory_with_format(include_bytes!($path2), image::ImageFormat::Png)
                .unwrap(),
            ImageRetentionPolicy::DropWhenUnusedForOneFrame,
        );
    };

    ($cx: expr, $path1:expr, @ $path2:expr) => {
        load_image!($cx, $path1, concat!("./editor/assets/", $path2));
    };
}

pub(crate) fn create(
    params: Arc<OneXOscParams>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);

        load_image!(cx,"sine-wave.png",@ "MdiSineWave.png");
        load_image!(cx,"triangle-wave.png",@ "MdiTriangleWave.png");
        load_image!(cx,"square-wave.png",@ "MdiSquareWave.png");
        load_image!(cx,"saw-wave.png",@ "MdiSawtoothWave.png");
        load_image!(cx,"noise.png",@ "MingcuteRandomFill.png");

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
