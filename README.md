
Compiling ruqslite.

If you get fatal linking error where the input file 'sqlite3.lib' cannot be found

You should probably install vckpg and go in the directory then run
`./vckpg install sqlite3:x64-windows`

Then add the directory where sqlite3.lib is located to cargo's config.toml: `${USER}/.cargo/config.toml` to let cargo know where to find the file for linking

We'll be using URLS a lot. This is a REST API after all.

`cargo add url`

Need to be able to serialize and deserialize the data types.

`cargo add serde`

- Error: cannot find derive macro `Deserialize` or `Serialize`
- Solution: enable the derive features in your project's `cargo.toml`
e.g.`serde = { version = "1.0", features = ["derive"] }`

We probably want to be able to serialize and deserialize into JSON because it is everywhere.

`cargo add serde_json`