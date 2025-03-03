use std::sync::Arc;

use nih_plug::prelude::{Editor};
use nih_plug::wrapper::vst3::vst3_sys::base::ClassCardinality;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};

use crate::param_knob::ParamKnob;
use crate::plugin::SchroederParams;


#[derive(Lens)]
struct Data {
    params : Arc<SchroederParams>
}

impl Model for Data{}

pub(crate) fn default_state() -> Arc<ViziaState>{
    ViziaState::new(||(600, 250))
}

// const SMALLER_FONT: nih_plug_egui::egui::FontId = FontId::proportional(11.0);
// const DEFAULT_SAMPLE_RATE : f32 = 44100.0;
// pub const TEAL_GREEN: Color32 = Color32::from_rgb(61, 178, 166);
// pub const DARKEST_BOTTOM_UI_COLOR: Color32 = Color32::from_rgb(27, 27, 27);
// pub const DARKER_GREY_UI_COLOR: Color32 = Color32::from_rgb(34, 34, 34);
// pub const DARK_GREY_UI_COLOR: Color32 = Color32::from_rgb(42, 42, 42);
// pub const MEDIUM_GREY_UI_COLOR: Color32 = Color32::from_rgb(52, 52, 52);
// pub const LIGHTER_GREY_UI_COLOR: Color32 = Color32::from_rgb(69, 69, 69);
// pub const A_BACKGROUND_COLOR_TOP: Color32 = Color32::from_rgb(38, 38, 38);

pub const TEAL_GREEN: Color = Color::rgb(61, 178, 166);
pub const YELLOW_MUSTARD: Color = Color::rgb(172, 131, 25);
pub const YELLOW_MUSTARD_PALID: Color = Color::rgba(172, 131, 25, 30);
pub const DARK_GREY_UI_COLOR: Color = Color::rgb(42, 42, 42);
pub const DARKER_GREY_UI_COLOR: Color = Color::rgba(34, 34, 34,120);
pub const HEADER_BG_COLOR: Color = Color::rgb(27, 27, 27);


// child-space: 1s;
// font-size: 18;

const STYLE: &str = r#"
.param_knob {
    width: 100px;
    height: 100px;
}

label {
    color: rgb(172, 131, 25);
}

.header-label {
    color: rgb(61, 178, 166);
}

knob {
    width: 50px;
    height: 50px;   
}

knob .track {
    background-color: rgb(172, 131, 25);
}

.param-label {
    color: rgb(172, 131, 25);
}

.tick {
    background-color: rgb(172, 131, 25);
}

.main-gui {
    background-color: rgb(34, 34, 34);
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

            HStack::new(cx, |cx|{                            
                Label::new(cx, "SCHROEDER REVERB")
                .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                .font_weight(FontWeightKeyword::DemiBold)
                .font_size(24.0)
                .height(Pixels(50.0))
                .child_top(Stretch(1.0))
                .child_bottom(Stretch(1.0))
                .color(YELLOW_MUSTARD)
                .background_color(HEADER_BG_COLOR)
                .width(Stretch(1.0));
            })
            .height(Pixels(50.0))
            .bottom(Pixels(10.0));


            HStack::new(cx, |cx|{                            
                HStack::new(cx, |cx|{                            
                    ParamKnob::new(cx, Data::params, |params| &params.rt60, false);
                    ParamKnob::new(cx, Data::params, |params| &params.dampening, false);
                    ParamKnob::new(cx, Data::params, |params| &params.mod_freq, false);
                    ParamKnob::new(cx, Data::params, |params| &params.dry_wet_mix, false);
                }).width(Stretch(0.75))        
                .background_color(YELLOW_MUSTARD_PALID)
                .child_top(Pixels(30.0))
                .space(Pixels(10.0));            
            }).background_color(DARKER_GREY_UI_COLOR);
           
       }).background_color(DARK_GREY_UI_COLOR);
       
       ResizeHandle::new(cx);

    })
}