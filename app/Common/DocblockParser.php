<?php

namespace App\Common;

use PHPStan\PhpDocParser\Ast\PhpDoc\PhpDocNode;
use PHPStan\PhpDocParser\Lexer\Lexer;
use PHPStan\PhpDocParser\Parser\PhpDocParser;
use PHPStan\PhpDocParser\Parser\TokenIterator;

final class DocblockParser
{
    public function __construct(
        private Lexer $lexer,
        private PhpDocParser $parser,
    ) {}

    public function parse(?string $docblock): ?PhpDocNode
    {
        if ($docblock === null) {
            return null;
        }

        return $this->parser->parse(
            new TokenIterator($this->lexer->tokenize($docblock))
        );
    }
}
