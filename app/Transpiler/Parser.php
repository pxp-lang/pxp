<?php

namespace App\Transpiler;

use Pxp\Parser\Lexer;
use Pxp\Parser\Lexer\Emulative;
use Pxp\Parser\Parser\Php7;
use Pxp\Parser\ParserFactory;
use Pxp\Parser\NodeTraverser;
use Pxp\Parser\NodeVisitor\NameResolver;
use Pxp\Parser\Parser as PxpParser;

final class Parser
{
    private Lexer $lexer;

    private PxpParser $parser;

    public function __construct()
    {
        $this->lexer = new Emulative([
            'usedAttributes' => [
                'comments', 'startLine', 'endLine', 'startFilePos', 'endFilePos',
            ],
        ]);

        $this->parser = (new ParserFactory)->create(ParserFactory::ONLY_PHP7, $this->lexer);
    }

    public function parse(string $code): array
    {
        $statements = $this->parser->parse($code);

        $traverser = new NodeTraverser();
        $traverser->addVisitor(new NameResolver());

        return $traverser->traverse($statements);
    }
}
