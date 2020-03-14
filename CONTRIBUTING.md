# Contributing

All sorts of contributions are welcome, no matter how large or small. Just fork
the repository, implement your change and make a pull request. Don't worry if
you are not sure how to implement the change, or if it's not yet done. You can
still make a work-in-progress pull request where we can discuss it.

Have a shiny new feature in store? Remember to explain it thoroughly and provide
motivation for why it should be added. This makes it easier to review and for
everyone to follow your reasoning.

## Testing

Every pull request is automatically tested with continuous integration to deny
warnings and any missing documentation. It's a good idea to run your local tests
with `RUSTFLAGS="-D warnings" cargo test` and also to run `cargo check` and 
`cargo build` with the compiler flag prepended. This will make sure that there
are no warnings or missing documentation, all for the benefit of the user.

There are a number of programs in the `examples` directory that that
demonstrate applications of the library in more "real" code. The output of
these should be checked to see if they are affected by changes made.
`readme_examples.rs` shall contain the same code as in the `README.md`
examples, and its output images shall be copied to the `gfx` directory (`cp
examples/readme_*.png gfx`) if they have changed.

### Unit Tests

New features should include appropriate unit tests to prevent future bugs.
This is especially important when it comes to dynamic things, like proper
conversion or data validation. The unit tests should be placed in a `test`
module located within the same module as the code being tested. For example:

```rust
struct Person {
    coolness: f32,
}

impl Person {
    pub fn is_cool(&self) -> bool {
        self.coolness >= 1.0
    }
}

//...

#[cfg(test)]
mod test {
    use super::Person;

    #[test]
    fn is_cool() {
        //...
    }
}
```

### Regression Tests

Each time a bug is fixed, a test of some sort (most probably a unit test)
should be added to check that the reported bug has been fixed in the reported
use case. This is to prevent the bug from reappearing in the future. The test
case may, of course, be expanded to check for more than just the reported case.

## Commits

Commits shall preferably be small and not contain too many different changes.
This makes them easier to cherry pick if necessary. The actual size of the
commit depends on the change, so "follow your heart", but it's preferable to
make sure that all the tests pass before committing.

The commit messages themselves don't need to have any particular formatting
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
and describe the changes as well as possible. The PR description (or initial
comment, depending on how you view it) should contain a relatively detailed
description of the changes. Someone who doesn't know what's up should be able
to look at it and understand what has changed. No secrets or surprises, even
if they may be fun.

Pull requests that close issues need to mention it in the description. A
closed issue should be mentioned as "fixes #123", "closes #123", or [something
similar][closing_commits]. This closes the issues automatically when the pull
request is merged.

Pull requests that break backwards compatibility should say so in the end of
the description, to make sure it's easy to find.

Here is an example PR:

>### Translate the library to British English
>
>The whole library is translated to the one and only Queen's English.
>
>This closes #123, by changing `Color` to `Colour`.
>
>This is a breaking change, since it changes the name of a number of
>identifiers.

It's not much harder than that, depending on the size of the contribution.

[closing_commits]: https://help.github.com/articles/closing-issues-via-commit-messages/

## Code Style

The code style is generally in line with the common guidelines for Rust. Running
`rustfmt` or `cargo fmt` before committing is recommended.

### Long Lines

The recommended line length is somewhere around 80 to 100 characters.

#### Documentation, Comments, And Text

Comments and the text in Markdown files are usually capped to 80 characters.
This can be done automatically in some editors like SublimeText (Alt+Q). Some
editors allow for visual rulers to indicate an 80 character width. It
doesn't hurt if this isn't followed as a law. It's just a guideline.
