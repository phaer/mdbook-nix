# mdbook-nix 

EXPERIMENTAL: This isn't finished, and might not ever be. At this point it's
a concept to discuss with friends and strangers.

Is a [mdBook][mdbook] pre-processor and [Rust][rust] library to run and verify
[Nix][nix] code blocks inside your documentation. It aims to make it easy to
keep your tutorials up to date and re-using them as a poor persons integration
test-suite.

We do so by parsing your [Markdown][gfm] files, looking for matching code blocks,
their "[info string][gfm-info-string]" and existing results.

If your files don't include results yet, i.e. on the first run, we evaluate the
code blocks and insert the output into your files.

On subsequent runs, we re-evaluate code blocks and compare existing output
to the new one, using [insta.rs][insta], to ensure everything still works.

# TODOS

- Snapshotting isn't ~~implemented~~ polished yet, we don't provide a cli yet.
  Use with `[INSTA_UPDATE](insta-update)` set to `always` on first run and `new` on subsequent
  rus. Also set `INSTA_OUTPUT` to `none` as that would break JSON output.
  Use `cargo insta review` to accept or reject new snapshots.
- Currently depends on wip rexpect PR at https://github.com/rust-cli/rexpect/pull/103.
- Make sessions configurable via `CodeBlockInfo.attributes`.
- Add bash support.
- Publish example book on Github pages.
- ~~Doesn't end the build on errors yet, but prints error inside the book.~~

# Inspiration

* [tesh](https://github.com/OceanSprint/tesh)
* [termbook](https://github.com/Byron/termbook)


[mdbook]: https://rust-lang.github.io/mdBook/
[rust]: https://www.rust-lang.org/
[nix]: https://nixos.org/
[gfm]: https://github.github.com/gfm/#what-is-github-flavored-markdown-
[gfm-info-string]: https://github.github.com/gfm/#info-string
[insta]: https://insta.rs/
[insta-update]: https://insta.rs/docs/advanced/#controlling-snapshot-updating
