# Overview
This crate defines a simple templating language in the format `%(SELECTOR)` that
can operate on a structure. A given `SELECTOR` accesses a given structure of
type `T` via a closure `Fn(&T) -> Option<String>`. Using a closure allows
complex logic to accessing the structure e.g. "if field _x_ does not exist,
then try field _y_, otherwise return a default value or `None`".

(This library is most definitely _not_ production-ready)

## Terminology
- A _template_ is a string `%(X)` where `X` is a _selector_
- A _selector_ is any string (including the empty string) without a round, closing parenthesis `)`
- The closure associated with a given _selector_ is known as an _accessor_
- A _format string_ is a string containing zero or more templates with other non-templating characters e.g. `%(id) - %(title)`

## Quick Start
In `Cargo.toml`:
```
[dependencies]
struct_string_template = "0.1.0"
```
Say we have a `struct` definition, and an instance of the `struct`:
```rust
struct Book {
    id: i64,
    title: String,
    author: String,
    contributors: Option<String>
}

let my_book = Book {
    id: 9784832275492,
    title: "Hidamari Sketch".to_owned(),
    author: "蒼樹うめ".to_owned(),
    contributors: None,
};
```
Define a format string:
```rust
let format_string = "[%(id)] %(title) %(所有作者)";
```
Build a `Templater<Book>` by doing:
```rust
use struct_string_template::TemplaterBuilder;

let templater = TemplaterBuilder::<Book>::new()
    .with_selector("id", |book| Some(book.id.to_string()))
    .with_selector("title", |book| Some(book.title.clone()))
    .with_selector("所有作者", |book| {
        Some(format!("(By: {}{})",
            &book.author,
            &book.contributors.clone().map(|x| format!(", {}", x)).or(Some("".to_owned())).unwrap())
        )
    })
    .build();
```
Render it using `Templater`'s `render` function
```rust
let result = templater.render(&my_book, format_string).ok().unwrap();
println!("{}", &result);
```

## Advanced Use

### Building `Templater` without `TemplateBuilder`
If the `Templater` needs to be built iteratively instead of using the builder
class, use `Templater::new()`, then add closures using the `insert` or `extend`
methods.
```rust
let mut templater = Templater::<Book>::new();
templater.insert("id", |book| Some(book.id.to_string()));
templater.insert("title", |book| Some(book.title.clone()));
templater.insert("所有作者", |book| {
    Some(format!("(By: {}{})",
        &book.author,
        &book.contributors.clone().map(|x| format!(", {}", x)).or(Some("".to_owned())).unwrap())
    )
});
```

### Formatter
If you plan to use a format string many times, you can "precompile" it similar
to a regex for better performance using the `Formatter` class (`render` turns
the format string into a `Formatter` internally), then pass the `Formatter`
variable into the `renderf` function:
```rust
use struct_string_template::Formatter;

let formatter = Formatter::build(format_string).ok().unwrap();
let result = templater.renderf(&my_book, &formatter);
println!("{}", &result);
```

## Error Handling
See `src/err.rs` for the errors thrown.

## Template Rules
In this section, `X` is any _selector_

1. `%%` is treated as a literal `%`
2. `%` followed by a character (or end of string) that is not `(` is invalid
3. `%(X)` is always valid
4. `%(X)A` where `A` is any valid string is valid
5. `%(X` (template is not terminated) is invalid
6. Having no closure associated with `X` is invalid
7. If the return value of the closure of `X` on the structure is `None`, `NA` is printed. This is currently unconfigurable except by modifying the closure itself.

## Limitations
Currently the `Templater` object cannot be copied around due to the closure
types. I've not found a work-around for this yet.

## Motivation
I use this library in a few personal applications of mine, and I've found it
annoying to keep changes to the library in sync between them.
