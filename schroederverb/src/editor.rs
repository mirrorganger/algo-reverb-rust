use std::sync::Arc;

use nih_plug::prelude::{Editor};
use nih_plug::wrapper::vst3::vst3_sys::base::ClassCardinality;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};

use crate::knob::ParamKnob;
use crate::plugin::SchroederParams;


#[derive(Lens)]
struct Data {
    params : Arc<SchroederParams>
}

impl Model for Data{}

pub(crate) fn default_state() -> Arc<ViziaState>{
    ViziaState::new(||(700,300))
}

const STYLE: &str = r#"
.param_knob {
    width: 100px;
    height: 100px;
}

label {
    child-space: 1s;
    font-size: 18;
    color: #BF9FF3;
}

.header-label {
    color: #EAEEED;
}

knob {
    width: 50px;
    height: 50px;   
}

knob .track {
    background-color: #9359F3;
}

.param-label {
    color: #EAEEED;
}

.tick {
    background-color: #9359F3;
}

.main-gui {
    background-color: rgb(170, 50, 50);
}

"#;



pub(crate) fn create(
    params : Arc<SchroederParams>,
    editor_state : Arc<ViziaState>
) -> Option<Box<dyn Editor>>{
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx,_|{
       assets::register_noto_sans_light(cx);
       assets::register_noto_sans_light(cx);
        
       cx.add_stylesheet(STYLE).unwrap();

       Data {
        params : params.clone()
       }.build(cx); 


       VStack::new(cx, |cx|{
            Label::new(cx, "Schroeder Reverb")
            .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
            .font_weight(FontWeightKeyword::Thin)
            .font_size(30.0)
            .height(Pixels(50.0))
            .child_top(Stretch(1.0))
            .child_bottom(Pixels(0.0));

            HStack::new(cx, |cx|{
                
                ParamKnob::new(cx, Data::params, |params| &params.rt60, false)
                .height(Pixels(60.0));


                ParamKnob::new(cx, Data::params, |params| &params.dampening, false)
                .height(Pixels(60.0));

                ParamKnob::new(cx, Data::params, |params| &params.mod_freq, false)
                .height(Pixels(60.0));

                ParamKnob::new(cx, Data::params, |params| &params.dry_wet_mix, false)
                .height(Pixels(60.0));
            }).col_between(Pixels(80.0));




       })
       .row_between(Pixels(0.0))
       .child_left(Stretch(1.0))
       .child_right(Stretch(1.0))
       .class("main-gui");
        ResizeHandle::new(cx);

    })
}