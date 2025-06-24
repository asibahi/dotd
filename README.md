# Dua of the Day

Tiny shell utility to start the terminal session with a small dua (prayer).

Expects a `.prayers` file in the home directory, where each line is a little dua. Then it just needs to be installed with `cargo install` and added to `.zshrc`.

If you are using a different terminal which displays Arabic text correctly, that's fine. If your terminal is Ghostty, which displays Arabic text reversed, make sure to pass in `--rev`.
