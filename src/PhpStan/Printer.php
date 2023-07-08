<?php

namespace Pxp\PhpStan;

use PHPStan\Node\Printer\Printer as BasePrinter;
use Pxp\Parser\Node\Expr\ShortClosure;

class Printer extends BasePrinter
{
    public function pExpr_ShortClosure(ShortClosure $node): string
    {
        return $this->pAttrGroups($node->attrGroups, true)
             . ($node->static ? 'static ' : '')
             . 'fn ' . ($node->byRef ? '&' : '')
             . '(' . $this->pCommaSeparated($node->params) . ')'
             . (null !== $node->returnType ? ' : ' . $this->p($node->returnType) : '')
             . ' {' . $this->pStmts($node->stmts) . $this->nl . '}';
    }
}
