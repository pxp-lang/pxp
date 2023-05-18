<?php

namespace App\Indexer;

use App\Common\DocblockParser;
use App\Common\File;
use App\Indexer\Visitors\IndexingVisitor;
use Pxp\Parser\NodeTraverser;
use RecursiveDirectoryIterator;
use RecursiveIteratorIterator;

final class Indexer
{
    public function __construct(
        private CachingParser $parser,
        private DocblockParser $docblockParser,
    ) {}

    public function index(array $paths = []): Index
    {
        if (! in_array(base_path('php-8-stubs'), $paths)) {
            $paths[] = base_path('php-8-stubs');
        }

        $index = new Index();

        foreach ($paths as $path) {
            $files = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($path, RecursiveDirectoryIterator::SKIP_DOTS));

            /** @var \SplFileInfo $file */
            foreach ($files as $file) {
                if ($file->getExtension() !== 'php' && $file->getExtension() !== 'pxp') {
                    continue;
                }

                $this->process($file->getPathname(), $index);
            }
        }

        return $index;
    }

    private function process(string $file, Index $index): void
    {
        $contents = file_get_contents($file);
        $statements = $this->parser->parse($contents);

        $traverser = new NodeTraverser();

        $traverser->addVisitor(new IndexingVisitor(
            file: new File($file),
            index: $index,
            docblockParser: $this->docblockParser,
        ));

        $traverser->traverse($statements);
    }
}
