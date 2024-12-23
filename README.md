# Tether egui UI Builder

A tiny desktop app for remote-controlling [Tether](https://github.com/RandomStudio/tether)-based systems, simulating input, and more (soon). Built using the immediate-mode GUI library, [egui](https://www.egui.rs/).

![GUI screenshot](tether-egui.gif)

## Easy start

### Install
On a Mac with homebrew, `brew install RandomStudio/tap/tether-egui` should do it.

Alternatively, if you have Cargo installed on your system, you can install using `cargo install tether-egui`.

...Or download the latest [release](https://github.com/RandomStudio/tether-egui/releases).

### Run
Simply launch from the command-line any time using `tether-egui`.

## Command-line options
By default, Tether Egui will try to connect to an MQTT broker running at `tcp://localhost:1883`
Launch without Tether (build your UI only) by passing `--tether.disable`

More options: `--help`
## Widgets available
- Floating-point Number (64bit)
- Whole Number (i64)
- Colour (8-bit for R,G,B,A)
- Boolean / Checkbox (e.g. for state)
- Empty Message (e.g for ping, heartbeat or representing an "event")
- Point2D (e.g. for tracking data)
- Generic Data (Parse string as JSON -> MessagePack)

## TODO/Roadmap
See Issues for suggested new features. And add your own!
