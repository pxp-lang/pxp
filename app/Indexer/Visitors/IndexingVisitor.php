<?php

namespace App\Indexer\Visitors;

use App\Common\DocblockParser;
use App\Common\File;
use App\Common\Location;
use App\Common\Type\MixedType;
use App\Common\Type\TypeConverter;
use App\Indexer\Entities\FunctionEntity;
use App\Indexer\Entities\Parameter;
use App\Indexer\Index;
use Pxp\Parser\Node;
use Pxp\Parser\Node\Stmt\Function_;
use Pxp\Parser\NodeVisitorAbstract;

class IndexingVisitor extends NodeVisitorAbstract
{
    public function __construct(
        protected File $file,
        protected Index $index,
        protected DocblockParser $docblockParser,
    ) {}

    protected function indexFunction(Function_ $node): void
    {
        $parameters = [];

        foreach ($node->getParams() as $parameter) {
            $parameters[] = new Parameter(
                name: $parameter->var->name,
                type: TypeConverter::fromParserType($parameter->type),
                variadic: $parameter->variadic,
            );
        }

        $this->index->addFunction(
            $node->namespacedName->toString(), new FunctionEntity(
                namespacedName: $node->namespacedName->toString(),
                name: $node->name->toString(),
                parameters: $parameters,
                returnType: TypeConverter::fromParserType($node->returnType),
                returnsByRef: $node->returnsByRef(),
                location: new Location(
                    $this->file,
                    $node->getStartFilePos(),
                )
            )
        );
    }

    public function enterNode(Node $node)
    {
        if ($node instanceof Function_) {
            $this->indexFunction($node);
        }
    }
}
