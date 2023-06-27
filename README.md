# Les Archives Perrusse

Les textes des capsules de François Perusse sont archivés dans ce dépot, notemment ceux des _Deux Minutes du Peuple_.

Ce dépot est créé dans le but de:
- Permettre une recherche efficace des textes pour trouver une capsule aisément
- Permettre de référencer les capsules, pour rendre possible des discussions à leur propos

## File syntax

*This section is in english because my brains thinks better about
programming in English.*

Each file contains a _capsule_. The file syntax is mostly a subset of
fountain.

### Metadata

There is one metadata section per file, written in TOML. Its beginning
and end are signaled by the string `---` being written on a separate
line.

### Scene heading

Scene headings are all caps string with an empty line before and after
them.

### Actions

Actions are line of text, separated with an empty line before and
after them.

### Dialog

A dialog starts with the name of a character in all caps. They can
then be followed by any number of line of text. They finish with an
empty line.

The lines of text that make up a dialog can be of two types:

- Parenthetical
- Line

#### Parenthetical

They start and end with parenthesis.

#### Line

Lines are normal text. They can contain puns.

Puns are normal text surrounded by square bracket. An id can be
provided to them. The ID is surrounded by square bracket and appended
to the pun.
