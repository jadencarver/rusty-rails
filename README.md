# Rusty Rails

A Rails-inspired framework for web development in Rust.

To start a new rusty-rails project, run:

```
PROJECT="project-name" && git clone https://github.com/jadencarver/rusty-rails.git -b latest "$PROJECT" -o rusty-rails && cd "$PROJECT" && git checkout -b master && cargo build
```

NOTE: It is recommended that you have binstubs configured, eg `export PATH="./bin:$PATH")`, if not prefix commands with `./bin/`.

You will most likely want to update the `.env` file with the appropriate database credentials.

## Starting the Server

```
start
```

And start exploring!  The server will automatically recompile the source code when necessary.

## Generators

Three generators are currently available:

- scaffold
- model
- controller

You can invoke the generators using the `generate` command.

```
generate scaffold blog title description
```

Rusty-Rails will make a good guess at the correct type, but you can also specify them explicitly.

```
generate scaffold blog title:string description:text
```

Be aware that specifying fields explicitly will make the fields private by default, so you must specify pub
in addition to the type if you intend them to be publicly accessible.

```
generate scaffold blog pub:title:string pub:description created_at:timestamp updated_at:timestamp
```

## Updates

To apply upstream changes to your project, simply run:

```
git fetch -a && git merge rusty-rails/latest
```

## Issues

Rusty-Rails is known to build correctly using the toolchain specified in [.rust-version](.rust-version).
If you experience issues compiling, and are using rustup.rs, run:

```
rustup override set `cat .rust-version`
```
