use std::sync::Arc;

use nih_plug::params::Param;
use nih_plug::prelude::{Editor};
use nih_plug::wrapper::vst3::vst3_sys::base::ClassCardinality;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};

use crate::audio_knob::KnobWidget;
use crate::plugin::SchroederParams;
use crate::colors::*;


#[derive(Lens)]
struct Data {
    params : Arc<SchroederParams>
}

impl Model for Data{}

pub(crate) fn default_state() -> Arc<ViziaState>{
    ViziaState::new(||(600, 300))
}

pub(crate) fn create(
    params : Arc<SchroederParams>,
    editor_state : Arc<ViziaState>
) -> Option<Box<dyn Editor>>{
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx,_|{
       assets::register_noto_sans_light(cx);
       assets::register_noto_sans_light(cx);
        
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
                .width(Stretch(1.0));
            })
            .height(Pixels(50.0))
            .bottom(Pixels(10.0));


            HStack::new(cx, |cx|{                            
                HStack::new(cx, |cx|{                            
                    KnobWidget::new(cx, Data::params, |params| &params.rt60, false);
                    KnobWidget::new(cx, Data::params, |params| &params.dampening, false);
                    KnobWidget::new(cx, Data::params, |params| &params.dry_wet_mix, false);
                    Binding::new(cx, Data::params.map(|val| val.mod_enabled.value()), |cx, lens| {
                        let value = lens.get(cx);
                        if value {
                            KnobWidget::new(cx, Data::params, |params| &params.mod_freq, false);
                        };
                    });  
            
            
                }).width(Stretch(0.75))        
                .child_top(Pixels(30.0))
                .height(Pixels(100.0));
            });

            ParamButton::new(cx, Data::params,  |params| &params.mod_enabled)
            .space(Stretch(0.1));
           
       }).row_between(Pixels(0.0))
       .background_color(DARK_GREY_UI_COLOR);
       
       ResizeHandle::new(cx);

    })
}