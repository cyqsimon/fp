# fp

fp (FilterPath) is the filter equivalent for the Unix `test` util.

## Quick start

```sh
$ ls
file-foo dir-read-write dir-read-only

# list files in the current directory, and filter for writable directories
$ ls | fp -dw
dir-read-write
```

For help, run `fp` with `-?` or `--help`.

The accepted filter flags are the same ones used by Unix `test`.
See [POSIX documentation](https://pubs.opengroup.org/onlinepubs/9699919799/utilities/test.html)
or [`man test`](https://manned.org/man/test).
