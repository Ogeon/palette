# Contributing

All sorts of contributions are welcome, no matter how huge or tiny. Just fork
the repository, implement your change and make a pull request. Don't worry if
you are not sure how to implement the change, or if it's not yet done. You can
still make an work-in-progress pull request where we can discuss it.

Got a new shiny feature in store? Remember to explain it thoroughly and
motivate why it should be added. This makes it easier to review and for
everyone to follow your reasoning.

## Testing

Every pull request is automatically tested with the `strict` feature enables,
so it's a good idea to also run your local tests with it, like this: `cargo
test --features strict`. This will make sure that there are no warnings or
missing documentation. All for the benefit of the user.

There are also a number of examples in the `examples` directory, that should
check that demonstrates things in a more "real" application. The output of
these should also be checked if they may be affected by the change.
`readme_examples.rs` shall contain the same code as in the `README.md`
examples, and its output images shall be copied to the `gfx` directory (`cp
examples/readme_*.png gfx`) if they have changed.

## Commits

Commits shall preferably be small and not contain too many different changes.
This makes them easier to cherry pick if that happens to be necessary. The
actual size of the commit depends on the change, so "follow your heart", but
it's preferable to make sure that all the tests passes before committing.

The commit messages themselves doesn't have to have any particular formating
or syntax (except English). Just make them short, _descriptive_ (!), and tidy.
Not like this:

 * fix
 * wip
 * wip 2

but rather like this:

 * Add the missing saturation validation in Hsv
 * Make Color implement the Mix trait

Notice how they are written as if they were instructions. Try not to write
them in past tense.

## Pull Requests

The header of a pull request should follow the same rules. It should be short
and describe the changes as good as possible. The PR description (or initial
comment, depending on how you view it) should contain a relatively detailed
description of the changes. Someone who doesn't know what's up should be able
to look at it and understand what has changed. No secrets or surprises, even
if they may be fun.

Pull requests that closes issues has to mention it in the description. A
closed issue should be mentioned as "fixes #123", "closes #123", or [something
similar][closing_commits]. This closes the issues automatically when the pull
request is merged.

Pull requests that breaks backwards compatibility should say so in the end of
the description, to make sure it's easy to find.

Here is an example PR:

>### Translate the library to British English
>
>The whole library is translated to the one and only Queen's English. This
>closes #123, by changing `Color` to `Colour`.
>
>This is a breaking change, since it changes the name of a number of
>identifiers.

It's not much harder than that, depending on the size of the contribution.

[closing_commits]: https://help.github.com/articles/closing-issues-via-commit-messages/
