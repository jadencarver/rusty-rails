# Rusty Rails

A Rails-inspired framework for web development in Rust.

To start a new rusty-rails project, run:

```
git clone https://github.com/jadencarver/rusty-rails.git -b latest [project-name]
git checkout -b master
```

And start exploring!  Some generators are currently supported as well:

```
generate scaffold blog title description
```

Rusty-rails will make a good guess at the correct type, but you can also specify them explicitly.

```
generate scaffold blog title:string description:text
```

Specifiying the explicitly however will make the fields private by default, so you must specify pub
if you intend them to be publically visible.

```
generate scaffold blog pub:title:string pub:description created_at:timestamp updated_at:timestamp
```
