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

### Unit Tests

New features should include appropriate unit tests to prevent future buggs.
This is especially important when it comes to dynamic things, like propper
conversion or valitation. The uni tests should be placed in a `test` module,
in the same module as the code that is being tested. For example:

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
This makes them easier to cherry pick if that happens to be necessary. The
actual size of the commit depends on the change, so "follow your heart", but
it's preferable to make sure that all the tests passes before committing.

The commit messages themselves doesn't have to have any particular formatting
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
>The whole library is translated to the one and only Queen's English.
>
>This closes #123, by changing `Color` to `Colour`.
>
>This is a breaking change, since it changes the name of a number of
>identifiers.

It's not much harder than that, depending on the size of the contribution.

[closing_commits]: https://help.github.com/articles/closing-issues-via-commit-messages/

## Code Style

The code style is generally in line with the common guidelines for Rust. There is a `rustfmt.toml` in the project, so feel free to use automatic formatting. If you prefer not to, the following sections can be used as a style guide.

### Braces

Opening braces should be placed on the same line as the control structure it
belongs to. `else` should be on the same line as the closing brace of `if`.

```rust
while x {

}

if a {

} else if b {

} else {

}

{
    do_stuff_in_scope();
}
```

### Indentation

This rule is simple: _always_ indent with _four spaces_, and _only one step_
per "level". No tabs, not two spaces, no visual indentation. The reasons for
this are:

1. Spaces are the Rust standard. Spaces are also predictable, so no surprise rightwards drift.
2. Fixed length indentation is less confusing for text editors. Some of them can easily be thrown off by irregular indentation lengths.
3. It's GIT friendly. Visual indentation depends on variable name lengths and other arbitrary things, so changing one of those things will affect more lines than necessary and cause noisy diffs.

### Long Lines

The recommended line length is somewhere around 80 to 100 characters, but
longer is allowed if it doesn't hurt readability. These are some guidelines
for how to deal with long lines.

#### Comments And Text

Comments and the text in Markdown files are usually capped to 80 characters.
This can be done automatically in some editors, like SublimeText (Alt+Q). It
doesn't hurt if this isn't followed as a law. It's just a guideline.

#### Call Chains

Long call chains can be broken up into multiple lines. Each call should then
be placed on its own line, and indented one step. It's as if they are placed
in a sub-scope. It's still ok, and often good, to have one method call on the
first line:

```rust
let b: Vec<_> = a.iter().map(From::from).filter(|x| x.prop() > 0.0).skip(10).take(100).collect();

//becomes

let b: Vec<_> = a.iter()
    .map(From::from)
    .filter(|x| x.prop() > 0.0)
    .skip(10)
    .take(100)
    .collect();

//or

let b: Vec<_> = a.iter()
    .map(From::from)
    .filter(|x| { //notice the braces and indentation
        x.prop() > 0.0
    })
    .skip(10)
    .take(100)
    .collect();
```

#### Function Signatures

Function signatures poses a risk of becoming very long. Especially when type
parameters are involved. Spreading them out over multiple lines follows the
same rules as above: everything is a scope, so everything is indented only
_one_ step. Here is a demo function and multiple stages of making it vertical:

```rust
fn long_function_with_a_long_name<'a, A: ?Sized + 'a, B: 'a, C, D>(abacus: &'a A, botanical: &'a B, culinary: Arc<Mutex<C>>, dependency: (D, D)) -> (D, D) where B: AsRef<A>, C: DerefMut, <C as Deref>::Target: RefMut<A> {
    //...
}

//`where` can be seen as an opening brace:
fn long_function_with_a_long_name<'a, A: ?Sized + 'a, B: 'a, C, D>(abacus: &'a A, botanical: &'a B, culinary: Arc<Mutex<C>>, dependency: (D, D)) -> (D, D) where
    B: AsRef<A>,
    C: DerefMut,
    <C as Deref>::Target: RefMut<A>,
{
    //...
}

//The argument list can be reformatted like the where clauses:
fn long_function_with_a_long_name<'a, A: ?Sized + 'a, B: 'a, C, D>(
    abacus: &'a A,
    botanical: &'a B,
    culinary: Arc<Mutex<C>>,
    dependency: (D, D),
) -> (D, D) where
    B: AsRef<A>,
    C: DerefMut,
    <C as Deref>::Target: RefMut<A>,
{
    //...
}

//This is quite extreme, but it follows the same rules:
fn long_function_with_a_long_name<
    'a,
    A: ?Sized + 'a,
    B: 'a,
    C,
    D
>(
    abacus: &'a A,
    botanical: &'a B,
    culinary: Arc<Mutex<C>>,
    dependency: (D, D),
) -> (D, D) where
    B: AsRef<A>,
    C: DerefMut,
    <C as Deref>::Target: RefMut<A>,
{
    //...
}
```

Notice how each section (or list) is clearly separated from each other. This
way of rewriting a function signature is, of course, just a guideline and
modifications are allowed for the sake of readability, but that should
generally not be necessary.

#### Input Arguments

The rules for input arguments are the same as above. Opening and closing
parenthesis, brackets and braces are separate from the content, and the content is
indented _one step_ only:

```rust
do_something_cool([abacus, botanical, culinary, dependency], |x| x.prop() > 0.0);

//becomes

do_something_cool(
    [
        abacus,
        botanical,
        culinary,
        dependency,
    ],
    |x| x.prop() > 0.0,
);
```
