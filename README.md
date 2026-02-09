Moved to https://codeberg.org/Lonami/makewav. The GitHub repository may be deleted in the future.

---

# makewav

A tiny program to generate simple sounds you can add to soundless recordings.

## Usage

Modify the `sample` function in [`main.rs`] and run the program with `cargo`.

Make sure to pipe the output to a file or another program or you'll get a bunch
of garbage in your terminal.

```sh
cargo run > beep.wav
```

You can also use `aplay` to play it directly:

```sh
cargo run | aplay
```

## Future directions

Command-line parameters to configure the duration, sample rate, and perhaps
even the function to generate the audio from could be really interesting.

[`main.rs`]: ./src/main.rs
