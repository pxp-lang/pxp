<?php

namespace Pxp\Transpiler\Visitors;

use Pxp\Parser\Node;
use Pxp\Parser\Node\Expr\Assign;
use Pxp\Parser\Node\Expr\Variable;
use Pxp\Parser\NodeVisitorAbstract;

class VariableFindingVisitor extends NodeVisitorAbstract
{
    protected array $variables = [];

    protected array $assignments = [];

    public function enterNode(Node $node)
    {
        if ($node instanceof Assign && $node->var instanceof Variable && is_string($node->var->name)) {
            $this->assignments[] = $node->var->name;
        }

        if ($node instanceof Variable && is_string($node->name) && ! in_array($node->name, $this->variables) && ! in_array($node->name, $this->assignments)) {
            $this->variables[] = $node->name;
        }

        return null;
    }

    public function getVariables(): array
    {
        return $this->variables;
    }
}
