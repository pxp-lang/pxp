# PXP

This repository contains the main PXP command-line application that powers the transpiler, static analyser and language server.

## Installation

> **Warning** - PXP is not ready for any form of production use.

You can install PXP in your project using Composer:

```sh
composer require pxp/pxp
```

This will expose a `vendor/bin/pxp` binary.

## Usage

Before using PXP, you must initialise your project.

```sh
vendor/bin/pxp init
```

This will create a configuration file in the current directory called `pxp.json`. The contents of this file are below.

```json
{
    "paths": [
        "app"
    ],
    "transpiler": {
        "sourceMap": false,
        "strictTypes": false
    }
}
```

### Configuration

The `pxp.json` file is used to configure all of PXP's processes. The table below describes each of the configuration options and whether they have been implemented yet.

| Option | Description | Implemented? |
| --- | --- | --- |
| `paths` | An array of directories or files that should be transpiled by PXP | ✅ |
| `transpiler.sourceMap` | Determines whether a source map should be generated when transpiling. This is used by PXP to ensure exceptions are accurate. | ❌ |
| `transpiler.strictTypes` | Whether a `declare(strict_types=1)` should be automatically added to the generated PHP code. | ❌ |

### Transpiling code

To transpile your PXP code to PHP, use the `pxp build` command.

```
vendor/bin/pxp build
```

This will use the `paths` from the configuration file to locate PXP files and generate the necessary PHP code.

The current version of PXP will store the PHP code **next to the PXP code** i.e. `app/User.pxp` will generate an `app/User.php` file.

## Contributing

All pull requests are welcome.

## Credit

* [Ryan Chandler](https://github.com/ryangjchandler)
