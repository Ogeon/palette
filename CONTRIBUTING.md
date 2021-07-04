# Contributing

All sorts of contributions are welcome, no matter how large or small. Just fork the repository, implement your change and make a pull request. Don't worry if you are not sure how to implement the change, or if it's not yet done. You can still make a work-in-progress pull request where we can discuss it.

Have a shiny new feature in store? Remember to explain it thoroughly and provide motivation for why it should be added. This makes it easier to review and for everyone to follow your reasoning.

## Testing

Every pull request is automatically tested with continuous integration to deny warnings and any missing documentation. It's a good idea to run your local tests with `RUSTFLAGS="-D warnings" cargo test` and also to run `cargo check` and `cargo build` with the compiler flag prepended. This will make sure that there are no warnings or missing documentation, all for the benefit of the user.

There are a number of programs in the `examples` directory that demonstrate applications of the library in more "real" code. The output of these should be checked to see if they are affected by changes made.

### Unit Tests

New features should include appropriate unit tests to prevent future bugs. This is especially important when it comes to dynamic things, like proper conversion or data validation. The unit tests should be placed in a `test` module located within the same module as the code being tested. For example:

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

Each time a bug is fixed, a test of some sort (most likely a unit test) should be added to check that the reported bug has been fixed in the reported use case. This is to prevent the bug from reappearing in the future. The test case may, of course, be expanded to check for more than just the reported case.

## Commits

Commits should be reasonably small and as self contained as possible. If there are fixup commits\* in a pull request, after review and eventual corrections, you will usually be asked to squash them into other commits.

The commit messages themselves don't need to have any particular formatting or syntax (except English). Just make them short, _descriptive_ (!), and tidy.

Not like this:

* fix
* wip
* wip 2

but rather like this:

* Add the missing saturation validation in Hsv
* Make Color implement the Mix trait

Notice how they are written as if they were instructions. Try not to write
them in past tense.

\* Fixup commits are any commits that fix mistakes in other commits within the same pull request. Squashing them into the "original" commits makes the history easier to follow.

## Pull Requests

The header/title of a pull request (PR) should follow the same rules as for commit messages. It should be short and describe the changes as well as possible. The PR description (a.k.a. initial comment, depending on how you view it) should contain a relatively detailed description of the changes. Someone who doesn't know what's going on should be able to look at it and understand what has changed. No secrets or surprises, even if they may be fun.

Both the title and the description will be embedded in the merge commit, and also used as source material for compiling release notes, so it's greatly appreciated if they are as clear and descriptive as possible.

Pull requests that close issues need to mention it in the description. A closed issue should be mentioned as "fixes #123", "closes #123", or [something similar][closing_commits]. This closes the issues automatically when the pull request is merged.

Pull requests that break backwards compatibility should say so in the end of the description, to make sure it's easy to find.

Here is an example PR:

>## Translate the library to British English
>
>The whole library is translated to the one and only Queen's English.
>
>### Closed Issues
>
>* This closes #123, by changing `Color` to `Colour`.
>
>### Breaking Change
>
>This changes the name of a number of identifiers to their British spelling.

That's about it, depending on the size of the contribution.

[closing_commits]: https://docs.github.com/en/free-pro-team@latest/github/managing-your-work-on-github/linking-a-pull-request-to-an-issue#linking-a-pull-request-to-an-issue-using-a-keyword

## Code Style

The code style is whatever `rustfmt` produces. Running `rustfmt` or `cargo fmt` before committing, or while editing, is therefore strongly recommended.

### Long Lines

`rustfmt` will typically take care of line wrapping, but in cases where it can't, the recommended line length is somewhere around 80 to 120 characters. Try to prioritize readability and break up any complex expressions that complicate formatting.

### Documentation

There are lints in place to make documentation a requirement. Remember to keep beginners in mind and add examples wherever possible.

Documentation comments are usually capped to 80 characters for source readability. This can be done automatically in some editors like SublimeText's Alt+Q, or via plugins like [Rewrap][rewrap] for Visual Studio Code. Some editors allow for visual rulers to indicate an 80 character width.

[rewrap]: https://marketplace.visualstudio.com/items?itemName=stkb.rewrap
