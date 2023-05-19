<?php

namespace Pxp\PhpStan;

use function str_ends_with;

class RichParser extends \PHPStan\Parser\RichParser
{
    public function __construct(
		private \PhpParser\Parser $parser,
		private Lexer $lexer,
		private NameResolver $nameResolver,
		private Container $container,
	)
	{
	}

    public function parseFile(string $file): array
    {
        if (! str_ends_with($file, '.pxp')) {
            return parent::parseFile($file);
        }
    }
}
