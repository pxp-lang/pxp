<?php

namespace Pxp\LanguageServer;

use Pxp\Parser\ErrorHandler\Collecting;
use Pxp\Parser\NodeTraverser;
use Pxp\Parser\ParserFactory;
use Pxp\Parser\Lexer\Emulative;
use Pxp\Parser\NodeVisitor\NameResolver;
use Pxp\Parser\Lexer;
use Pxp\Parser\Parser;

final class TolerantParser
{
    private Lexer $lexer;

    private Parser $parser;

    private Collecting $errorHandler;

    public function __construct()
    {
        $this->lexer = new Emulative([
            'usedAttributes' => [
                'comments', 'startLine', 'endLine', 'startFilePos', 'endFilePos',
            ],
        ]);

        $this->errorHandler = new Collecting();
        $this->parser = (new ParserFactory)->create(ParserFactory::ONLY_PHP7, $this->lexer);
    }

    public function parse(string $code): array
    {
        $statements = $this->parser->parse($code, errorHandler: $this->errorHandler);

        $traverser = new NodeTraverser();
        $traverser->addVisitor(new NameResolver(errorHandler: $this->errorHandler));

        return $traverser->traverse($statements);
    }
}
