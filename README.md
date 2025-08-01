# Dua of the Day

Tiny shell utility to start the terminal session with a small dua (prayer).

Expects a `.prayers` file in the home directory, where each line is a little dua. Then it just needs to be installed with `cargo install` and added to `.zshrc`.

Different terminals have different ways they fuck up Arabic text. Ghostty reverses the whole string and shows it backwards. Zed terminal shows every singular word correctly but the string is reversed. Terminal.app works fine.

For Ghostty, use `--rev`. For Zed Terminal, use `--rev-words`. If your terminal mucks Arabic text in a different way from these two please submit a PR.
