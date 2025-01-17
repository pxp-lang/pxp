<?php

namespace Pxp;

use FilesystemIterator;
use PhpParser\Node;
use PhpParser\Node\ComplexType;
use PhpParser\Node\Identifier;
use PhpParser\Node\Name;
use PhpParser\Node\Param;
use PhpParser\Node\Stmt\Function_;
use PhpParser\NodeTraverser;
use PhpParser\NodeVisitor\NameResolver;
use PhpParser\NodeVisitorAbstract;
use PhpParser\Parser;
use PhpParser\ParserFactory;
use RecursiveDirectoryIterator;
use RecursiveIteratorIterator;
use SplFileInfo;

require_once __DIR__ . '/../vendor/autoload.php';

final class Processor
{
    private RecursiveIteratorIterator $stubs;

    private array $entities = [];

    private Parser $parser;

    public function __construct()
    {
        $this->stubs = new RecursiveIteratorIterator(new RecursiveDirectoryIterator(__DIR__ . '/../stubs', FilesystemIterator::SKIP_DOTS));
        $this->parser = (new ParserFactory)->createForNewestSupportedVersion();
    }

    public function process(): void
    {
        foreach ($this->stubs as $stub) {
            $this->stub($stub);       
        }
        
        dd($this->entities);
    }

    private function stub(SplFileInfo $file): void
    {
        if ($file->getFilename() === 'LICENSE') {
            return;
        }

        $contents = file_get_contents($file->getRealPath());
        
        $traverser = new NodeTraverser(new NameResolver(options: [
            'preserveOriginalNames' => true,
        ]));
        
        $ast = $traverser->traverse($this->parser->parse($contents));

        $traverser = new NodeTraverser(new EntityLocatingVisitor($this->entities));

        $traverser->traverse($ast);
    }
}

final class EntityLocatingVisitor extends NodeVisitorAbstract
{
    public function __construct(private array &$entities) {}

    public function enterNode(Node $node)
    {
        if ($node instanceof Function_) {
            $this->processFunction($node);
        }
    }

    private function processFunction(Function_ $node): void
    {
        $this->entities[] = [
            'type' => 'FunctionEntity',
            'name' => [
                'resolved' => $node->namespacedName->toString(),
                'original' => $node->name->toString(),
            ],
            'parameters' => $this->processParameters($node->getParams()),
            'return_type' => $this->processType($node->getReturnType()),
            'returns_reference' => $node->returnsByRef(),
            'location' => [
                'file' => 0,
                'span' => [$node->getStartFilePos(), $node->getEndFilePos()],
            ],
        ];
    }

    /**
     * @param array<Param> $parameters
     */
    private function processParameters(array $parameters): array
    {
        return array_map(function (Param $param) {

        }, $parameters);
    }

    private function processType(Identifier|Name|ComplexType|null $type): ?array
    {
        if ($type === null) {
            return null;
        }

        return [];
    }
}

$processor = new Processor();
$processor->process();
