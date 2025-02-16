# Schroeder reverb plugin written in Rust

Reverb audio plugin written in Rust. 

It implements the classic [Schroeder](https://ccrma.stanford.edu/~jos/pasp/Schroeder_Reverberators.html) reverb algorithm.

The plugin uses the [nih-plug](https://github.com/robbert-vdh/nih-plug.git)  audio plugin framework to produce VST3 and Clap plugins.

### Building

First, get a hold of the [euterpe-rs](https://github.com/mirrorganger/euterpe-rs.git) submodule use by the project:

```bash
git submodule update --init
```

Then, build the plugin using the following command:


```bash
cargo xtask bundle schroederverb --release
```

Then, copy the plugin into your system's VST3 directory. For example, in Linux:

```bash
cp -r target/bundle/schroederverb.vst3 ~/.vst3
```

You should be able to load the plugin in your DAW.
