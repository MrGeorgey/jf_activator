# JustFlight Addon Activator

Small rust program that copies userinfo.txt from a modified JustFlight aircraft and moves it to the respective folder for MSFS2020/2024 Steam/Microsoft Store variants.

## Building

Since this is a `cargo` project, it can be easily built with

```powershell
cargo build
```

or for release

```powershell
cargo build --release
```

## Configuration

To configure this for a new aircraft, place the **jf_activator.exe** and **jf_activator.toml** found in `target\debug` or `target\release` depending on how you [built](#building) the program in the root folder of the aircraft to be activated, and then edit the **jf_activator.toml** file to represent the details of the aircraft and it's version.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
