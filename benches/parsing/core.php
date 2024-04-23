<?php

$directory = $argv[1];
$iterator = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($directory, FilesystemIterator::SKIP_DOTS));
$count = 0;

/** @var SplFileInfo $file */
foreach ($iterator as $file) {
    if ($file->getExtension() !== 'php') {
        continue;
    }

    $code = file_get_contents($file->getRealPath());

    try {
        ast\parse_code($code, 100);
    } catch (Error) {
        // Ignore
    }

    $count++;
}

echo "{$count} files parsed\n";