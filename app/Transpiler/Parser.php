<?php

namespace Pxp\Transpiler;

use PhpParser\Lexer;
use Pxp\Parser\Lexer\Emulative;
use PhpParser\NodeTraverser;
use PhpParser\NodeVisitor\NameResolver;
use PhpParser\Parser as ParserInterface;
use Pxp\Parser\Parser\Pxp;

final class Parser
{
    private Lexer $lexer;

    private ParserInterface $parser;

    public function __construct()
    {
        $this->lexer = new Emulative([
            'usedAttributes' => [
                'comments', 'startLine', 'endLine', 'startFilePos', 'endFilePos',
            ],
        ]);

        $this->parser = new Pxp($this->lexer);
    }

    public function parse(string $code): array
    {
        $statements = $this->parser->parse($code);

        $traverser = new NodeTraverser();
        $traverser->addVisitor(new NameResolver());

        return $traverser->traverse($statements);
    }
}
