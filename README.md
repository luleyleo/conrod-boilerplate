# Getting started with Conrod

## Why this repository exists
This is a cleaner and documented rewrite of the template that I personally use when I write an app using Conrod. I've written this to make it easier to get started with Conrod (or just start a new project) and because many people complained about the current state of the guide (righteously)

In order to be flexible Conrod leaves a lot of choices (and thus work) to you an requires a fair amount of boilerplate to get started.

This repository provides a starting point, implemented in the way I personally use Conrod.
You can develop an application with Conrod using any model you like, mine is inspired by React.js and thus this boilerplate makes it ease to do this.

And while this template gives an idea of how to structure an application that uses Conrod, it wont teach you about how layout so forth work, for this you can use the examples of Conrod.

## Its structure
The basic structure of a Conrod app looks like this:
```
- project
    - src
        - components
            - app.rs
        - boiler.rs
        - eventloop.rs
        - main.rs
    - Cargo.toml
```

### The main file
I prefer to keep my `main.rs` as clean as possible and use it only for `extern crate` and `mod` declarations.
The main function simply looks like this:
```rust
main() {
    boiler::boil();
}
```

### The eventloop
This file is literally a copy of the end of the `examples/support/mod.rs` file of Conrod and contains the `EventLoop` struct which keeps the app running.  It works and that is all I care about.

### The boiler
This file is a lot more interesting. It is a copy of `examples/counter.rs` modified to simply render my `App`.
Those are the interesting parts of this file:
- window title and dimensions
- fonts
- images
- render the `App`

Using the `TITLE`,  `INITIAL_WINDOW_WIDTH` and `INITIAL_WINDOW_HEIGHT` statics/contants you can control those properties **at startup**.

> How you deal with multiple fonts or any images is not *yet* included in this template but I intend to do this in a branch.

### The app
This is the heart of your application.

#### The simple case
In a simple application this is where everything happens.
By adding stuff to the `State` struct you can preserve well... state.
And your apps ui will be where the `Button` is currently created.

#### The complex case
In this case your `App` component should stay as simple as possible, otherwise things can get messy.
It will basically connect your apps pieces. Anything that has even just some logic and state should go into its own `component` which basically is just a custom `Widget` just like the `App`.
For example if you have a todo app that consists out of a `Header`, `List` and `Footer` you'll probably end up with the following structure:

```rust
- project
    - src
        - components
            - app.rs
            - header.rs
            - list.rs
            - todo_item.rs
            - footer.rs
        - models
            - todo.rs
        - boiler.rs
        - eventloop.rs
        - main.rs
    - Cargo.toml
```

The header job will be to provide a `TextField` so a user can write down a new todo and a `Button` to submit it. Thus the headers `State` will have a field `input: String` and emit `Option<Todo>` as Event.

The list will render all todos as `TodoItem`s that it gets passed in a list and emit a `Vec<TodoListEvent>`
```rust
enum TodoListEvent {
    Toggled(Id),
    Removed(Id),
    Updated(Id, String),
}
```

A `TodoItem` shows a `Toggle` whether the task is completed, a `Button` to delete it and lets you edit its content. Thus it will emit a `Vec<TodoItemEvent>`
```rust
enum TodoItemEvent {
    Toggled,
    Removed,
    Updated(String),
}
```
that can be picked up by the `TodoList` and, combined with its Id, be sent further up to the `App`.

The `Footer` lets the user filter the todos (All / Open / Closed) and thus shows for example three `Toggle`s and emits an `Option<TodoFilter>`
```rust
enum TodoFilter {
    All,
    Open,
    Closed
}
```

And then the `App` combines those three to an actually useful piece.
The `App` holds a `Vec<Todo>` and `TodoFilter` in its State.
Then it then creates the three parts and deals with their events.
```rust
fn update(...) {
    for new_todo in Header::new()
        .do_some_layout()
        .set(state.ids.header, ui)
    {
        state.update(|state| state.todos.push(new_todo));
    }

    for event in List::new(state.todos.iter().filter(state.filter.check))
        .do_some_layout()
        .set(state.ids.list, ui)
    {
        match event {
            TodoListEvent::Toggled(id) => do_stuff(),
            ...
        }
    }

    for new_filter in Footer::new()
        .do_some_layout()
        .set(state.ids.footer, ui)
    {
        state.update(|state| state.filter = new_filter);
    }
}
```
