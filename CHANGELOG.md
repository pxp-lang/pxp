## Changelog

This documents contains all changes made to the repository between releases.

## `v0.0.2`

* Add basic support for PHPStan.

This release includes **very basic** support for analysing PXP files with PHPStan.

We provide our own PHPStan extension that allows PHPStan to analyse PXP files. The approach to this involves replacing and extending some of PHPStan's own internal classes so we can't guarantee that it's stable at the moment.

Another limitation with the current approach is that if you have run `pxp build` to transpile your code, PHPStan will analyse the PXP file as well as the generated PHP. We hope to fix this in a future release by programatically excluding generated PHP files.

## `v0.0.1`

* Initial release.
