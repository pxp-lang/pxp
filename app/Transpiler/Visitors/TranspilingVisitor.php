<?php

namespace App\Transpiler\Visitors;

use App\Transpiler\Transformers\ShortClosureTransformer;
use Pxp\Parser\Node;
use Pxp\Parser\Node\Expr\ShortClosure;
use Pxp\Parser\NodeVisitorAbstract;

class TranspilingVisitor extends NodeVisitorAbstract
{
    public function leaveNode(Node $node)
    {
        if ($node instanceof ShortClosure) {
            return ShortClosureTransformer::transform($node);
        }

        return null;
    }
}
