# Flumox

Flumox is a web application for hosting puzzle hunts and outdoor games.

It is usable, but unfinished.
Currently, the only way to prepare a game is by seeding it into the Postgres database manually.
A tool (`flumox-seed-maker`) exists to prepare such a seed.

## Time expressions

To express various conditions, such as whether a widget is visible or a hint is available, Flumox uses a custom expression language.

### Literals

There are three basic types of literals. There is `always`, which always true, and `never`, which is never true. There are also time literals, such as `2020-05-13 15:00 +2`, which trigger at a specific time.

### Paths

Paths can be used to refer to various events that happened during the game. For example, `first.solved` will be true when the prompt named `first` is solved, and `home-second.hint.spoiler.visible` will trigger when when the hint `spoiler` for the prompt `home-second` becomes visible.

### Operators

The expression `a & b` will be true once both `a` and `b` are true. `a | b` will be true once either `a` or `b` are true.

### Offsets

The expression language also supports delaying events. For example, `a + 15 m` will become true fifteen minutes after `a` becomes true.
