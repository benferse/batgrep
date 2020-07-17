[fzf.vim](https://github.com/junegunn/fzf.vim) includes a script that can be used to invoke 
[bat](https://github.com/sharkdp/bat) to preview files. However, it's a bourne shell script,
which doesn't work by default on Windows. This is really only a problem for search results from
something like [ag](https://github.com/ggreer/the_silver_searcher) since the format of the results
includes line and column information that bat doesn't understand by default.

This is a small utility that can be used in its place that should theoretically work wherever it
can be compiled.

## License

`SPDX-License-Identifier: MIT`
