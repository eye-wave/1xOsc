use nih_plug::params::{EnumParam, Param};
use nih_plug_vizia::{vizia::prelude::*, widgets::param_base::ParamWidgetBase};

use super::icons::{
    Icon, Point, NOISE_ICON_POINTS, SAW_ICON_POINTS, SINE_ICON_POINTS, SQUARE_ICON_POINTS,
    TRIANGLE_ICON_POINTS,
};
use crate::model::osc::OscillatorType;

enum RadioEvent {
    Set(OscillatorType),
}

#[derive(Lens)]
pub struct BtnGroup {
    param_base: ParamWidgetBase,
    current_state: OscillatorType,
}

impl View for BtnGroup {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|state_change, _| match state_change {
            RadioEvent::Set(variant) => {
                let enum_param = EnumParam::new("__temp_osc_type", *variant);

                self.param_base
                    .set_normalized_value(cx, enum_param.preview_normalized(*variant));
                self.current_state = *variant;
            }
        })
    }
}

impl BtnGroup {
    pub fn new<L, Params, P, FMap>(
        cx: &mut Context,
        params: L,
        params_to_param: FMap,
        initial_state: OscillatorType,
    ) -> Handle<Self>
    where
        L: Lens<Target = Params> + Clone + Copy,
        Params: 'static,
        P: Param + 'static,
        FMap: Fn(&Params) -> &P + Copy + 'static,
    {
        Self {
            param_base: ParamWidgetBase::new(cx, params, params_to_param),
            current_state: initial_state,
        }
        .build(cx, |cx| {
            HStack::new(cx, |cx| {
                Binding::new(cx, Self::current_state, |cx, state| {
                    ([
                        (OscillatorType::Sine, SINE_ICON_POINTS.to_vec()),
                        (OscillatorType::Triangle, TRIANGLE_ICON_POINTS.to_vec()),
                        (OscillatorType::Square, SQUARE_ICON_POINTS.to_vec()),
                        (OscillatorType::Sawtooth, SAW_ICON_POINTS.to_vec()),
                        (OscillatorType::Noise, NOISE_ICON_POINTS.to_vec()),
                    ] as [(OscillatorType, Vec<Point>); 5])
                        .iter()
                        .for_each(|(variant, points)| {
                            let variant = *variant;
                            let points = points.clone();

                            HStack::new(cx, move |cx| {
                                Icon::new(cx, points.clone());
                            })
                            .width(Pixels(48.0))
                            .height(Pixels(48.0))
                            .on_mouse_up(move |cx, _| cx.emit(RadioEvent::Set(variant)))
                            .checked(state.map(move |v| v.same(&variant)))
                            .class("osc-btn");
                        });
                });
            });
        })
    }
}
