<?php

/**
 * This file is used to generate versioned stub directories from [phpstan/php-8-stubs](https://github.com/phpstan/php-8-stubs).
 * 
 * The stubs that the package provides used #[Until] and #[Since] attributes to separate things between versions. This script
 * will generate a directory for each version and copy the stubs that are relevant to that version.
 * 
 * The script is invoked by the crate's build script and the generated directories are used to generate version-specific indexes.
 */

use PhpParser\Node;
use PhpParser\Node\Attribute;
use PhpParser\NodeTraverser;
use PhpParser\NodeVisitor;
use PhpParser\NodeVisitorAbstract;
use PhpParser\Parser;
use PhpParser\ParserFactory;
use PhpParser\PrettyPrinter\Standard;

require_once __DIR__.'/vendor/autoload.php';

const VERSIONS = [
    '8.0',
    '8.1',
    '8.2',
    '8.3',
];

function main() {
    $files = [];
    $stubDirectory = __DIR__ . '/vendor/phpstan/php-8-stubs/stubs';

    foreach (new RecursiveIteratorIterator(new RecursiveDirectoryIterator($stubDirectory)) as $file) {
        if ($file->isFile() && $file->getExtension() === 'php') {
            $files[] = $file->getPathname();
        }
    }

    printf("Found %d files.\n", count($files));

    foreach (VERSIONS as $version) {
        generateStubsForVersion($version, $files);
    }

    printf("Done.\n");
}

function generateStubsForVersion(string $version, array $files) {
    printf("Generating stubs for version %s.\n", $version);

    $versionDirectory = __DIR__ . '/../stubs/' . $version;

    if (! is_dir($versionDirectory)) {
        mkdir($versionDirectory, recursive: true);
    }

    $parser = (new ParserFactory)->createForNewestSupportedVersion();
    $printer = new Standard();

    foreach ($files as $file) {
        $ast = parseAndFilterFileForVersion($parser, $file, $version);

        if ($ast === []) {
            continue;
        }

        $versionedStubPath = $versionDirectory . '/' . str_replace(__DIR__ . '/vendor/phpstan/php-8-stubs/stubs/', '', $file);

        if (! is_dir(dirname($versionedStubPath))) {
            mkdir(dirname($versionedStubPath), recursive: true);
        }

        file_put_contents($versionedStubPath, $printer->prettyPrintFile($ast));
    }
}

function parseAndFilterFileForVersion(Parser $parser, string $file, string $version): array
{
    $ast = $parser->parse(file_get_contents($file));
    $traverser = new NodeTraverser(new VersionSpecificNodeRemover($version));
    
    return $traverser->traverse($ast);
}

class VersionSpecificNodeRemover extends NodeVisitorAbstract
{
    public function __construct(
        private string $version,
    ) {}

    function enterNode(Node $node)
    {
        if (! property_exists($node, 'attrGroups')) {
            return;
        }

        if ($node->attrGroups === []) {
            return;
        }

        $sinceVersion = $this->tryFindVersionFromAttributes('Since', $node->attrGroups);
        $untilVersion = $this->tryFindVersionFromAttributes('Until', $node->attrGroups);

        if ($sinceVersion === null && $untilVersion === null) {
            return;
        }

        if ($sinceVersion !== null && version_compare($sinceVersion, $this->version, '>')) {
            return NodeVisitor::REMOVE_NODE;
        }

        if ($untilVersion !== null && version_compare($untilVersion, $this->version, '<')) {
            return NodeVisitor::REMOVE_NODE;
        }

        return $this->removeAttributesFromNode($this->removeAttributesFromNode($node, 'Since'), 'Until');
    }

    function removeAttributesFromNode(Node $node, string $name): Node
    {
        $groups = $node->attrGroups;

        foreach ($groups as $group) {
            $group->attrs = array_filter($group->attrs, fn($attr) => $attr->name->toString() !== $name);
        }

        $node->attrGroups = array_filter($groups, fn($group) => count($group->attrs) > 0);

        return $node;
    }

    function tryFindVersionFromAttributes(string $name, array $attrGroups): ?string
    {
        foreach ($attrGroups as $attrGroup) {
            foreach ($attrGroup->attrs as $attr) {
                if ($attr->name->toString() === $name) {
                    return $attr->args[0]->value->value;
                }
            }
        }

        return null;
    }
}

main();