# Declutter
A Habit Tracking application built with Rust and GTK, and integrated with the GNOME desktop.
This project is currently under development. It is not ready for use yet, but you can follow the progress here. Any help is welcome!
## Building the project

Make sure you have `flatpak` and `flatpak-builder` installed. Then run the commands below. Normally this would be handled by your IDE, such as GNOME Builder or VS Code with the Flatpak extension.

```
flatpak install org.gnome.Sdk//43 org.freedesktop.Sdk.Extension.rust-stable//22.08 org.gnome.Platform//43
flatpak-builder --user flatpak_app build-aux/com.amankrx.Declutter.Devel.json
```

## Running the project

Once the project is build, run the command below. 

```
flatpak-builder --run flatpak_app build-aux/com.amankrx.Declutter.Devel.json Declutter
```

## Community

Join the GNOME and gtk-rs community!
- [Matrix chat](https://matrix.to/#/#rust:gnome.org): chat with other developers using gtk-rs
- [Discourse forum](https://discourse.gnome.org/tag/rust): topics tagged with `rust` on the GNOME forum.
- [GNOME circle](https://circle.gnome.org/): take inspiration from applications and libraries already extending the GNOME ecosystem.

## Credits

- [Podcasts](https://gitlab.gnome.org/World/podcasts)
- [Shortwave](https://gitlab.gnome.org/World/Shortwave)
- [Health](https://gitlab.gnome.org/World/Health)
