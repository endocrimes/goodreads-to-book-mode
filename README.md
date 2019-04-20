# goodreads-to-book-mode

A command line tool that pulls read history from the GoodReads API and exports
it in a format readable by
[book-mode](https://github.com/spacekookie/book-mode).

## Usage

```bash
$ export USER_ID="<your-goodreads-user-id>"
$ export GOODREADS_TOKEN="<your-goodreads-api-token>"
$ cargo run > read.books
```

## Limitations

It currently only extracts title, read date, and isbn data. I may eventually
also include author data or try to extract more book data where available.
