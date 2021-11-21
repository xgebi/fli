# FLI

Flask CLI tool is a remake of a [project](https://github.com/xgebi/flask-gen-cli) from last year. At the moment it provides basic function of creating a new module.

## Usage

It has to be in PATH variable for it work as in examples below.

### Creating new package

```shell
fli new package <path>
```

When there is a path deeper than root directory the command will create whole path of directories and `__init__.py` and `routes.py` files at the end.

When the path contains only root directory, it will be created along with `__init__.py` file.

### Creating new file

```shell
fli new file -n <name> <path>
```

This command will create a new file called `<name>.py` in current directory when path is empty or at the specified directory.