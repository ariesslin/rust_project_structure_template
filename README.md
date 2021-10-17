# rust_project_structure_template


### CONCEPTS

- **Packages:** A <u>Cargo</u> feature that lets you build, test, and share crates
- **Crates:** A <u>tree of modules</u> that produces a library or executable
- **Modules** and **use:** Let you control the organization, scope, and privacy of paths
- **Paths:** A way of <u>naming</u> an item, such as a struct, function, or module



**A package** 

- contains a *Cargo.toml* file that describes how to build those crates.
- can contain at most one library crate. 
- can contain as many binary crates as you’d like, 
- but must contain at least one crate (either library or binary).



**A crate** is a binary or library. 

**The *crate root*** is a source file that the Rust compiler starts from and makes up the root module of your crate

A crate’s functionality is <u>namespaced</u> in its own scope. (use `::` to access a crate’s scope)



**Modules** let us organize code within a crate into groups for readability and easy reuse. Modules also <u>control the *privacy*</u> of items, which is whether an item can be used by outside code (*public*) or is an internal implementation detail and not available for outside use (*private*).

We use the keyword `mod` to define a module.

Inside modules, we can have other modules. Modules can also hold definitions for other items, such as structs, enums, constants, traits, or functions.

Earlier, we mentioned that *src/main.rs* and *src/lib.rs* are called crate roots. The reason for their name is that the contents of either of these two files form a module named `crate` at the root of the crate’s module structure, known as the *module tree*.

Modules aren’t useful only for organizing your code. They also define <u>Rust’s *privacy boundary*</u>: the line that encapsulates the implementation details external code isn’t allowed to know about, call, or rely on. So, if you want to make an item like a function or struct private, you put it in a module.

In Rust all items (functions, methods, structs, enums, modules, and constants) are <u>private by default</u>. Items in a parent module can’t use the private items inside child modules, but items in child modules can use the items in their ancestor modules.



**A path** is used to find an item in a module tree, it can take two forms:

- An *absolute path* starts from a crate root by using a crate name or a literal `crate`.
- A *relative path* starts from the current module and uses `self`, `super`, or an identifier in the current module.

Both absolute and relative paths are followed by one or more identifiers separated by double colons (`::`).

We can also construct relative paths that begin in the parent module by using `super` at the start of the path. This is like starting a filesystem path with the `..` syntax. 



### Cargo's conventions

- *<u>src/main.rs</u>* is the crate root of a binary crate with the same name as the package. 
- if the package directory contains *<u>src/lib.rs</u>*, the package contains a library crate with the same name as the package, and *src/lib.rs* is its crate root.
- If a package contains *src/main.rs* and *src/lib.rs*, it has two crates: a library and a binary, both with the same name as the package. 
- A package can have multiple binary crates by placing files in the *src/bin* directory: each file will be a separate binary crate.



### The `use` Keyword

 We can bring a path into a scope once and then call the items in that path as if they’re local items with the `use` keyword.

After the path, we can specify `as` and a new local name, or <u>alias</u>, for the type. 

When we bring a name into scope with the `use` keyword, the name available in the new scope is private. To enable the code that calls our code to refer to that name as if it had been defined in that code’s scope, we can combine `pub` and `use`. This technique is called *<u>re-exporting</u>* because we’re bringing an item into scope but also making that item available for others to bring into their scope.



### Separating Modules into Different Files

Using a semicolon after `mod mod_name` rather than using a block tells Rust to load the contents of the module from another file with the same name as the module. 

