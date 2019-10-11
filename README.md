# tmux-copy

A highly opinionated copying plugin inspired by several predecessors:

- [tmux-fingers][tf]
- [tmux-picker][tp]
- [tmux-thumbs][tt]

Meant primarily for personal use. Works on Linux and MacOS, relying
on [rust-clipboard][cb] for system clipboard support.

## Installation

Requires [Rust][rust] to compile the binaries, and [tpm][tpm]
(`tmux` Plugin Manager) to manage the `tmux` configuration.

- Add `set -g @plugin 'nwtnni/tmux-copy'` to your `.tmux.conf`
- Install with `<PREFIX>-I`
- Navigate to your plugin directory (typically `~/.tmux/plugins`)
- Run `cargo build --release` to compile

## Known Issues

- Not very configurable

- Requires Rust compiler to build binaries from source

- Doesn't support zoomed `tmux` panes

- Doesn't support scrolled `tmux` panes

- Doesn't handle Unicode: matches will be offset

- Allows overlapping matches

- Only supports up to 400 simultaneous hints

## Configuration

- Regular expressions defined in [find][find]

- Matching colors defined in [main][main]

- Hint strings defined in [hint][hint]

- Key binding (`<PREFIX>-f`) defined in [tmux-copy.tmux][tc]

## Performance

- Uses [regex][re] as the underlying search engine

- Scans each line once with a [RegexSet][rs] to determine if any specific
  regular expression matches. Scans again for each regular expression that
  matches at least once in the line. Assuming matches are sparse, we'll
  only scan each line a single time independently of how many specific
  regular expressions there are.

- Generates hint strings at compile time using a [fun macro][ct]

- Minimizes calls to `tmux`

[ct]: https://github.com/nwtnni/tmux-copy/blob/8fd1d3340f4628b45cf8998141db9bce69f9e715/src/util.rs#L1-L11
[tf]: https://github.com/Morantron/tmux-fingers
[tp]: https://github.com/pawel-wiejacha/tmux-picker 
[tt]: https://github.com/fcsonline/tmux-thumbs
[tc]: https://github.com/nwtnni/tmux-copy/blob/master/tmux-copy.tmux
[cb]: https://github.com/aweinstock314/rust-clipboard
[re]: https://docs.rs/regex/1.3.1/regex/
[rs]: https://docs.rs/regex/1.3.1/regex/struct.RegexSet.html
[tpm]: https://github.com/tmux-plugins/tpm
[boot]: https://github.com/nwtnni/tmux-copy/blob/master/src/boot.rs
[find]: https://github.com/nwtnni/tmux-copy/blob/master/src/find.rs
[hint]: https://github.com/nwtnni/tmux-copy/blob/master/src/hint.rs
[main]: https://github.com/nwtnni/tmux-copy/blob/master/src/main.rs
[rust]: https://rustup.rs/
