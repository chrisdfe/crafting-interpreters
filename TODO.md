# TODO

- [x] implement clap https://docs.rs/clap
- [x] Better parse errors (should show character, column, line)
- [ ] implement text_io for interactive shell https://docs.rs/text_io/latest/text_io/
- [ ] Better runtime errors (should show character, column, line)
- [ ] Look more into not having to clone things so much in parser/interpreter
- [ ] create a macro for defining builtins
- [ ] replace 'var' with 'let'
- [ ] program doesn't actually exit on a ParseErr
- [ ] currently environments stay around for the entirety of the program. Depending on what happens next with the book, either redo how they're implemented (Rc, Weak, RefCell, etc) or implement Drop on LiteralValues, or something
