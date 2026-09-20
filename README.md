# Polyread

An e-book reader with TTS and same-book multilingual support

## Usage

> polyread [options] <file>
available options:
  -h, --help            Show this help message
  -v, --version         Show version information
  -f, --format <format> Specify the format of the input file (txt, epub, pdf, docx)
  -l, --language <lang> Specify the language for TTS (e.g., en, es, fr)
  -s, --speed <speed>   Set the TTS speed (default: 1.0)
  -b, --bookmark        Add a bookmark at the current location
  -c, --chapter         Jump to a specific chapter
  -S, --search <query>  Search for a specific term in the book
  -p, --progress        Show reading progress
  -t, --tts <model>     Specify the TTS model to use (default: system TTS)

## key bind draft (v.1)

Global / App

    q / Esc - quit / back out of current overlay
    ? - toggle help panel

Cursor movement (semantic)

    h - previous sentence
    l - next sentence
    k - previous paragraph
    j - next paragraph

Viewport movement (cursor unchanged)

    Ctrl-u - half-page up
    Ctrl-d - half-page down
    PgUp - page up
    PgDn - page down
    MouseWheelUp - scroll up
    MouseWheelDown - scroll down

Selection / Highlight

    v - toggle selection mode
    In selection mode: h/l/j/k extend selection by sentence/paragraph
    Esc - clear selection / exit selection mode

Search

    / - open search input
    Enter - confirm search
    n - next search result
    p - previous search result

Chapters

    c - open chapter palette
    ]c - next chapter
    [c - previous chapter
    Enter in chapter palette - jump to selected chapter

Bookmarks

    m - add bookmark (selection range if active, else point bookmark)
    b - open bookmarks palette
    ]b - next bookmark
    [b - previous bookmark
    Enter in bookmark palette - jump to selected bookmark

TTS

    Space — play/pause TTS
    = (or +) — increase TTS speed
    - - decrease TTS speed
    s - speak current sentence / active selection (optional explicit speak)

Future-reserved

    . - repeat last action (reserve now, implement later)
    : - command palette/command line (optional future)
