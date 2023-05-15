<?php

namespace App\Transpiler\Transformers;

use App\Transpiler\Visitors\VariableFindingVisitor;
use Pxp\Parser\Node\Expr\ClosureUse;
use Pxp\Parser\Node\Expr\Closure;
use Pxp\Parser\Node\Expr\ShortClosure;
use Pxp\Parser\Node\Expr\Variable;
use Pxp\Parser\NodeTraverser;

class ShortClosureTransformer
{
    public static function transform(ShortClosure $node): Closure
    {
        $variableFindingVisitor = new VariableFindingVisitor;
        $traverser = new NodeTraverser;

        $traverser->addVisitor($variableFindingVisitor);
        $traverser->traverse($node->getStmts());

        $variables = $variableFindingVisitor->getVariables();
        $uses = [];

        foreach ($variables as $variable) {
            $uses[] = new ClosureUse(new Variable($variable), byRef: false);
        }

        return new Closure([
            'static' => $node->static,
            'byRef' => $node->returnsByRef(),
            'params' => $node->getParams(),
            'returnType' => $node->getReturnType(),
            'stmts' => $node->getStmts(),
            'uses' => $uses,
            'attrGroups' => $node->getAttrGroups(),
        ]);
    }
}
