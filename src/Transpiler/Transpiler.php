<?php

namespace Pxp\Transpiler;

use Pxp\Common\Configuration\TranspilerOptions;
use Pxp\Transpiler\Visitors\TranspilingVisitor;
use PhpParser\NodeTraverser;
use Pxp\Parser\PrettyPrinter\Standard;

final class Transpiler
{
    private Parser $parser;

    private Standard $printer;

    private NodeTraverser $traverser;

    public function __construct(Parser $parser)
    {
        $this->parser = $parser;
        $this->printer = new Standard();

        $this->traverser = new NodeTraverser();
        $this->traverser->addVisitor(new TranspilingVisitor);
    }

    public function transpile(string $code): string
    {
        $statements = $this->parser->parse($code);
        $statements = $this->traverser->traverse($statements);

        return $this->printer->prettyPrintFile($statements);
    }
}
