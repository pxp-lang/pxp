<?php

use PhpParser\Error;
use PhpParser\ParserFactory;

require_once __DIR__ . '/vendor/autoload.php';

$directory = $argv[1];
$iterator = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($directory, FilesystemIterator::SKIP_DOTS));
$parser = (new ParserFactory)->createForNewestSupportedVersion();
$count = 0;

/** @var SplFileInfo $file */
foreach ($iterator as $file) {
    if ($file->getExtension() !== 'php') {
        continue;
    }

    $code = file_get_contents($file->getRealPath());

    try {
        $parser->parse($code);
    } catch (Error) {
        // Ignore
    }

    $count++;
}

echo "{$count} files parsed\n";