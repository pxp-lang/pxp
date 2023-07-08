<?php

namespace Pxp\Commands;

use Pxp\Common\Configuration\Configuration;
use Pxp\Transpiler\Transpiler;
use LaravelZero\Framework\Commands\Command;
use RecursiveDirectoryIterator;
use RecursiveIteratorIterator;

class BuildCommand extends Command
{
    protected $signature = 'build';

    protected $description = 'Analyse and transpile PXP code to PHP.';

    public function handle(Configuration $configuration, Transpiler $transpiler)
    {
        $paths = $configuration->paths;
        $files = [];

        foreach ($paths as $path) {
            if (is_file($path)) {
                $files[] = realpath($path);
                continue;
            }

            if (! is_dir($path)) {
                continue;
            }

            $iterator = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($path, RecursiveDirectoryIterator::SKIP_DOTS));

            /** @var \SplFileInfo $file */
            foreach ($iterator as $file) {
                if ($file->getExtension() !== 'pxp') {
                    continue;
                }

                $files[] = $file->getPathname();
            }
        }

        foreach ($files as $file) {
            $transpiled = $transpiler->transpile(
                file_get_contents($file),
                $configuration->transpiler,
            );

            $destination = str_replace('.pxp', '.php', $file);

            file_put_contents($destination, $transpiled);
        }
    }
}
