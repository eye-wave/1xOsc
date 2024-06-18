use nih_plug::params::{EnumParam, Param};
use nih_plug_vizia::{vizia::prelude::*, widgets::param_base::ParamWidgetBase};

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
                    [
                        (OscillatorType::Sine, "sine-wave.png"),
                        (OscillatorType::Triangle, "triangle-wave.png"),
                        (OscillatorType::Square, "square-wave.png"),
                        (OscillatorType::Sawtooth, "saw-wave.png"),
                        (OscillatorType::Noise, "noise.png"),
                    ]
                    .iter()
                    .for_each(|(variant, src)| {
                        Image::new(cx, *src)
                            .on_mouse_up(|cx, _| cx.emit(RadioEvent::Set(*variant)))
                            .checked(state.map(|v| v.same(variant)))
                            .class("osc-btn");
                    });
                });
            });
        })
    }
}
