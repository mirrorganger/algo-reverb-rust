use nih_plug::prelude::{Param, Vst3SubCategory};
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::param_base::ParamWidgetBase;
use crate::colors::{self};
#[derive(Debug)]
pub enum ParamEvent{
    BeginSetParam,
    SetParam(f32),
    EndSetParam,
}

const STYLE: &str = r#"
.knob_widget {
    width: 150px;
}

knob {
    width: 100px;
    height: 100px;
}

.tick {
    color: #808080;
    background-color: rgb(52, 52, 52);
}

label {
    child-space: 1s;
    font-size: 20;
    color: #808080;
}

"#;


#[derive(Lens)]
pub struct KnobWidget {
    param_base: ParamWidgetBase,
}

impl KnobWidget {
    pub fn new<L, Params, P, FMap>(
        cx: &mut Context,
        params: L,
        params_to_param: FMap,
        centered: bool,
    ) -> Handle<Self>
    where
        L: Lens<Target = Params> + Clone + Copy,
        Params: 'static,
        P: Param + 'static,
        FMap: Fn(&Params) -> &P + Copy + 'static,
    {
        Self {
            param_base: ParamWidgetBase::new(cx, params.clone(), params_to_param),
        }.build(
            cx,
            ParamWidgetBase::build_view(params, params_to_param, move |cx, param_data| {
            cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");    
            VStack::new(cx , |cx|{
 
 
                Knob::custom(
                    cx,
                    param_data.param().default_normalized_value(),
                    params.map(move |params| {
                        params_to_param(params).unmodulated_normalized_value()
                    }),

                    move |cx, lens| {
                        TickKnob::new(
                            cx,
                            Percentage(50.0),
                            Pixels(4.),
                            Percentage(55.0),
                            300.0,
                            KnobMode::Continuous,
                        ).value(lens.clone())
                        .color(colors::YELLOW_VARIANT)
                        .background_color(colors::YELLOW_VARIANT);
                    
                        ArcTrack::new(
                            cx,
                            centered,
                            Percentage(95.0),
                            Percentage(20.),
                            -150.,
                            150.,
                            KnobMode::Continuous,
                        ).value(lens)
                        .color(colors::YELLOW_MUSTARD)
                        .background_color(colors::YELLOW_MUSTARD_PALID)
                        .class("track");
                    
                        Label::new(
                            cx,
                            params.map(move |params| {
                                 params_to_param(params).normalized_value_to_string(
                                    params_to_param(params)
                                        .modulated_normalized_value()
                                        .to_owned(),
                                    true,
                                )
                                .to_owned()
                            }),
                        ).color(colors::DARKER_GREY_UI_COLOR)
                        .font_weight(FontWeightKeyword::Bold)
                    
                    
                    },
                ).space(Stretch(1.0))
                .bottom(Stretch(0.))
                .on_mouse_down(move |cx, _button| {
                    cx.emit(ParamEvent::BeginSetParam);
                })
                .on_changing(move |cx, val| {
                    cx.emit(ParamEvent::SetParam(val));
                })
                .on_mouse_up(move |cx, _button| {
                    cx.emit(ParamEvent::EndSetParam);
                });


                Label::new(
                    cx,
                    params.map(move |params| params_to_param(params).name().to_owned()),
                )
                .space(Stretch(1.0))
                .top(Stretch(0.))
                .font_weight(FontWeightKeyword::Bold)
                .color(colors::YELLOW_VARIANT);
                }).class("knob_widget");
            }),
        )
    }
}


impl View  for KnobWidget {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|param_change_event, _| match param_change_event {
            ParamEvent::BeginSetParam => {
                self.param_base.begin_set_parameter(cx);
            }
            ParamEvent::SetParam(val) => {
                self.param_base.set_normalized_value(cx, *val);
            }
            ParamEvent::EndSetParam => {
                self.param_base.end_set_parameter(cx);
            }
        });
    }
}

