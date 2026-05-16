# rsh
**the small, simple rust shell**

rsh is a small, portable, secure and fast shell in Rust.

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

### donating

Donations are highly appreciated and help me work on this more often as it helps motivate me.

**Ko-Fi**: [@xorlaw](https://ko-fi.com/xorlaw)

### contributing

To contribute, open a pull request. Anything that can help this project be better is accepted.







