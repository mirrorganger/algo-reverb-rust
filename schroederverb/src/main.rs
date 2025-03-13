use nih_plug::prelude::*;

use schroederverb::plugin::SchroederPlugin;

fn main() {
    nih_export_standalone::<SchroederPlugin>();
}