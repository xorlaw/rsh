# rsh
**the small, simple rust shell**

rsh is a small, pretty crappy (currently) shell that is written in Rust, unlike most shells that are written in C.
This language change can provide some security benefits like memory safety.

> [!IMPORTANT]
> rsh is a passion project at the most, and is in pretty heavy development.
> I am also a solo developer. This means that this shell will probably not be the best shell out there. It could probably compete though.

### installing

> [!NOTE]
> If you use `doas` instead of `sudo`, edit the `SUDO = sudo` line near the top of the Makefile to `SUDO = doas`. You can also edit the other variables if you prefer to have a seperate install location.

The Makefile is designed to be pretty flexible. This section is basically just saying how to use it.

- `build` - Builds the project in Release mode. Does not install.
- `debug` - Builds the project in Debug mode. Does not install.
- `install` - Adds the built binary in `target/release` to `usr/local/bin` and `/etc/shells`. **Does not build.**
- `clean` - Cleans the built binary out of `target/release` or `target/debug`. **Does not remove anything from `/etc/shells` or `/usr/local/bin`.**
- `remove` - Removes the binary out of `/etc/shells` and `/usr/local/bin`.

If you are a regular user, run `make build`, then `make install`.


### todo

- [x] organize more
- [x] make somewhat useable
- [x] add aliases
- [ ] fix some code in `parser.rs` which breaks alot of aliases
- [ ] general improvements

### contributing

To contribute, open a pull request. Anything that can help this project be better is accepted.







