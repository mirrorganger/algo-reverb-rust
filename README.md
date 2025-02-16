# Schroeder reverb plugin written in Rust

Reverb audio plugin written in Rust. It implements the [classic Schroeder reverb](https://ccrma.stanford.edu/~jos/pasp/Schroeder_Reverberators.html) 

The plugin uses the [nih-plug](https://github.com/robbert-vdh/nih-plug.git)  audio plugin plugin framework to produce VST3 and Clap plugins.

### Bulding

First, get a hold of the [euterpe-rs](https://github.com/mirrorganger/euterpe-rs.git) submodule use by the project:

```bash
git submodule update --init
```

Then, build the plugin using the following command:


```bash
cargo xtask bundle schroederverb --release
```

Then, copy the plugin in the VST3 directory of your systemm. For example, in Linux:

```bash
cp -r target/bundle/schroederverb.vst3 ~/.vst3
```

You should be able to load the plugin in your DAW.
