<?php

namespace Pxp\Transpiler\Visitors;

use Pxp\Transpiler\Transformers\ShortClosureTransformer;
use PhpParser\Node;
use Pxp\Parser\Node\Expr\ShortClosure;
use PhpParser\NodeVisitorAbstract;

class TranspilingVisitor extends NodeVisitorAbstract
{
    public function leaveNode(Node $node)
    {
        $new = match (true) {
            $node instanceof ShortClosure => ShortClosureTransformer::transform($node),
            default => null,
        };

        if ($new === null) {
            return null;
        }

        $new->setAttribute('sourceNode', $node);

        return $new;
    }
}
