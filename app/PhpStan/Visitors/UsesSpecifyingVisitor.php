<?php

namespace Pxp\PhpStan\Visitors;

use PhpParser\Node;
use PhpParser\Node\Expr\ClosureUse;
use PhpParser\NodeTraverser;
use PhpParser\NodeVisitorAbstract;
use PhpParser\Node\Expr\Variable;
use Pxp\Parser\Node\Expr\ShortClosure;
use Pxp\Transpiler\Visitors\VariableFindingVisitor;

class UsesSpecifyingVisitor extends NodeVisitorAbstract
{
    public function enterNode(Node $node)
    {
        if ($node instanceof ShortClosure) {
            $traverser = new NodeTraverser();
            $visitor = new VariableFindingVisitor;

            $traverser->addVisitor($visitor);
            $traverser->traverse($node->stmts);

            foreach ($visitor->getVariables() as [$variable, $attributes]) {
                $node->uses[] = new ClosureUse(new Variable($variable, $attributes), byRef: false);
            }
        }

        return null;
    }
}
