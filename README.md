# pgls

**pgls** is a CLI tool to fetch and list links and domains from a given URL, with options for filtering internal or external links.

### Installation

You can quickly install Pgls by running the following command in your terminal:

```sh
curl -fsSL https://raw.githubusercontent.com/ranajahanzaib/pgls/main/install.sh | sudo bash
```

**Note**: Ensure you have sudo privileges as the script moves the binary to a system directory.

## Usage

The utility accepts the following command-line arguments:

```sh
pgls 0.1
A CLI tool to fetch and list links and domains from a given URL, with options for filtering internal or external links.

USAGE:
pgls [OPTIONS]

OPTIONS:
domains      # List all domains mentioned on the page
links        # List all links mentioned on the page
  --external # List only external links
  --internal # List only internal links

```

#### Example

List all links from a Wikipedia page:

```sh
pgls links https://en.m.wikipedia.org/
```

List all external links from a Wikipedia page:

```sh
pgls links https://en.m.wikipedia.org/ --external
```

List all domains from a Wikipedia page:

```sh
pgls domains https://en.m.wikipedia.org/
```

## Contributing

We’d love to accept your patches and contributions to this project. There are just a few guidelines you need to follow.

### [Code of Conduct](./CODE_OF_CONDUCT.md)

This project follows the [Contributor Covenant](https://www.contributor-covenant.org/) as its Code of Conduct, and we expect all project participants to adhere to it. Kindly read the [full guide](./CODE_OF_CONDUCT.md) to understand what actions will not be tolerated.

### [Contributing Guide](./CONTRIBUTING.md)

Read our [contributing guide](./CONTRIBUTING.md) to learn about our development process, how to propose bug fixes and improvements, and how to build and test your changes to the project.

### Issues

Feel free to submit issues and enhancement requests. Use the template provided when creating an issue to ensure your request is clear and actionable.

See the open issues for a complete list of proposed features (and known issues).

## [License](./LICENSE)

This project is licensed under the [MIT License](./LICENSE), meaning that you’re free to modify, distribute, and/or use it for any commercial or private project.
