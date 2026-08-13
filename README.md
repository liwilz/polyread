# Polyread

An e-book reader with TTS and same-book multilingual support

Road Map (For me)

## Phase 0: MVP (Minimum Viable Product)

- Be able to handle files like txt, epub, pdf, and docx etc.
  - At least 1 format

- Provide basic loading and scrolling through the book.

## Phase 1: Keyboard centric navigation

keybind draft (v1)

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

- Partly load files into memory for efficiency

- Use vim-like keybindings for navigation, selection (visual mode)

- Allow for auto scrolling and speed control

- QoL: Jump, Search, Bookmarking, Progress tracking, Last read location

## Phase 2: TTS (Text-to-Speech) Support

- Implement some TTS engine abstraction to support multiple TTS engines

- Implement a TTS engine

## Phase 3: Multilingual Support

- Basic support for having books in different languages (Single language per book)

- Implement manual selection of language for TTS

- Build underlying infrastructure to record and restore memory of language
  choice on selected text

  - Efficiency concern:
    - choice of language stored to track locations only
    - Buffer-like retrieval for memory concerns

- Extra keyboard controls to easily note language of selected text (e.g. macros)

## Phase 4

- Automatic language mapping using translation engines or AI models

- Note: Should not require an account either way: local or free remote solutions

- Note: Small LLMs are surprisingly easy to run, but it's unnecessary.

## Extra Features

Translation to user's preferred language?

Fun idea: Type to read
