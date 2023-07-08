<?php

namespace Pxp;

use Closure;
use Composer\Autoload\ClassLoader;
use Pxp\Transpiler\Parser;
use Pxp\Transpiler\Transpiler;

class Autoloader
{
    protected Transpiler $transpiler;

    public function __construct()
    {
        $this->transpiler = new Transpiler(new Parser);
    }

    public function autoload(string $class)
    {
        $loaders = ClassLoader::getRegisteredLoaders();

        if (count($loaders) === 0) {
            return;
        }

        // We need to grab the Composer autoloader so that we can use it to
        // figure out which namespaces are associated with which folder.
        $loader = $loaders[array_key_first($loaders)];

        $logicalPath = strtr($class, '\\', DIRECTORY_SEPARATOR) . '.pxp';
        $prefixLengths = Closure::bind(fn () => $this->prefixLengthsPsr4, $loader, $loader)();
        $prefixes = $loader->getPrefixesPsr4();

        if (isset($prefixLengths[$class[0]])) {
            $subPath = $class;

            while (false !== $lastPos = strrpos($subPath, '\\')) {
                $subPath = substr($subPath, 0, $lastPos);
                $search = $subPath . '\\';

                if (isset($prefixes[$search])) {
                    $pathEnd = DIRECTORY_SEPARATOR . substr($logicalPath, $lastPos + 1);
                    foreach ($prefixes[$search] as $dir) {
                        if (file_exists($file = $dir . $pathEnd)) {
                            $this->load($file);
                        }
                    }
                }
            }
        }
    }

    protected function load(string $file): void
    {
        $hash = md5_file($file);
        $cache = $this->getCachePath() . DIRECTORY_SEPARATOR . $hash . '.php';

        if (file_exists($cache)) {
            include $cache;
            return;
        }

        $transpiled = $this->transpiler->transpile(file_get_contents($file));

        file_put_contents($cache, $transpiled);

        include $cache;
    }

    protected function getCachePath(): string
    {
        return realpath(__DIR__ . '/../cache');
    }
}
