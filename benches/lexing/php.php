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
        PhpToken::tokenize($code, TOKEN_PARSE);
    } catch (Error) {

    }

    $count++;
}

echo "{$count} files lexed\n";