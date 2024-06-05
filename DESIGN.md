# Features

## Finds the right help

- Main command help
  - man pages
  - `cmd help | --help | -h | -?`
  - web page?
- Subcommand help
  - man pages
  - `cmd sub help | --help | -h | -?`

## Colorizes and formats the content

- Like [flan](https://github.com/vlymar/flan)

## Interactive command parsing

- Supply a command in the UI and watch it filter the help down to an exact match
- Supply a command when launching to pre-fill the interactive filter
  - Can just press Backspace to clear it and see the normal help content

## Natural language questions

- Answer any question about the command or its usage, with
  - Results returned as text, in the same UI
  - Highlight or reference other parts of the help text where possible
