# Polyread

An e-book reader with TTS and same-book multilingual support

Road Map (For me)

## Phase 0: MVP (Minimum Viable Product)

- Be able to handle files like txt, epub, pdf, and docx etc.
  - At least 1

- Provide basic loading and scrolling through the book.

## Phase 1: Keyboard centric navigation

- Use vim-like keybindings for navigation, selection (visual mode)

- Allow for auto scrolling and speed control

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

- Automatic language mapping using translation engines or LLMs
