# Welcome to switcher 👋
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)
[![Cargo: Switcher](https://img.shields.io/crates/v/switcher)](https://crates.io/crates/switcher)
[![Github: CI](https://img.shields.io/github/workflow/status/omarahm3/switcher/CI/master)](https://github.com/omarahm3/switcher/actions)
[![Twitter: omarahm3](https://img.shields.io/twitter/follow/omarahm3.svg?style=social)](https://twitter.com/omarahm3)

> Multi git repositories project organizer made with rust.

This project is pretty immature, it was created so that i can learn Rust, there are other variants of this project made with other languages
- [Fish Shell](https://github.com/omarahm3/projects-switcher)
- [Node Switcher](https://github.com/omarahm3/node-switcher)

However this might be the main project in which i'm going to continue maintaining it.

## Install

From releases

```sh
wget -qcO ~/.local/bin/switcher https://github.com/omarahm3/switcher/releases/download/v<LATEST_VERSION>/switcher
chmod +x ~/.local/bin/switcher
```

Or install it as a cargo crate

```sh
cargo install switcher --version <LATEST_VERSION>
```

If you just don't know the version to use

```sh
cargo install switcher --version (cargo search switcher --limit 1 | head -n 1 | cut -d ' ' -f 3 | sed -e 's/"//g')
```

## Build

```sh
git clone git@github.com:omarahm3/switcher.git
cd switcher
cargo build --release --all-features
```

## Usage

```sh
switcher help
```

### Using feature files
Feature files are files that describe a certain feature/bug/whatever on a specific project. You can create them in a specific format:

```json
{
	"project": "example",
	"feature_specs": [
		{
			"repository": "my_repo",
			"branch": "fix/tricky-bug"
		},
		...
	]
}
```
You can check [this example file](./examples/example_feature.json) too, and once you have the file somewhere you can just pass that to switcher by running:
```sh
switcher feature ./path-to-feature-file.json
```
and it will do the rest

## Author

👤 **Omar Ahmed**

* Website: https://mrg.sh
* Twitter: [@omarahm3](https://twitter.com/omarahm3)
* Github: [@omarahm3](https://github.com/omarahm3)
* LinkedIn: [@omarahmed0](https://linkedin.com/in/omarahmed0)

## 🤝 Contributing

Contributions, issues and feature requests are welcome!

Feel free to check [issues page](https://github.com/omarahm3/switcher/issues). 

## Show your support

Give a ⭐️ if this project helped you!


## 📝 License

Copyright © 2021 [Omar Ahmed](https://github.com/omarahm3).

This project is [MIT](./LICENSE) licensed.
