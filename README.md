# PXP

This repository contains the main PXP command-line application that powers the transpiler, static analyser and language server.

**PXP is not ready for production use, but we do encourage you to try it out and report any bugs or problems you face.**

## Installation

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

## PHPStan

PXP has **very basic** support for analysing your files with PHPStan. The extension comes with the `pxp/pxp` package and will be automatically loaded by PHPStan when using [`phpstan/extension-installer`](https://github.com/phpstan/extension-installer).

If you're not using `phpstan/extension-installer` you can manually load the extension:

```neon
includes:
    - ./vendor/pxp/pxp/app/PhpStan/extension.neon

parameters:
    // ...
```

**Known issues and potential limitations**

1. Since PHPStan is only designed to analyse regular PHP files, the PXP extension makes a few changes to the internal PHPStan classes, including the parser, lexer and pretty printer. This _could_ cause some instability, but based on some testing, everything seems okay. The classes being replaced are internal to PHPStan and therefore do not fall under any backwards compatibilty promises. If you do encounter a problem, please [report it in an issue](https://github.com/pxp-lang/pxp/issues/new).

2. If you have used the `pxp build` command to transpile and generate PHP code, PHPStan will start to analyse the original PXP code as well as the generated PHP file. We hope to resolve this issue in a future release by dynamically excluding PHP files that are associated with a PXP file.


## Contributing

All pull requests are welcome.

## Credit

* [Ryan Chandler](https://github.com/ryangjchandler)
