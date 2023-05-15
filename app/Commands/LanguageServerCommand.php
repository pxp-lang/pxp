<?php

namespace App\Commands;

use App\Common\Configuration\Configuration;
use App\LanguageServer\LanguageServerDispatcherFactory;
use Exception;
use Illuminate\Console\Scheduling\Schedule;
use LaravelZero\Framework\Commands\Command;
use Phpactor\LanguageServer\LanguageServerBuilder;
use Psr\Log\NullLogger;
use RuntimeException;
use Symfony\Component\VarDumper\Caster\ReflectionCaster;
use Symfony\Component\VarDumper\Cloner\Data;
use Symfony\Component\VarDumper\Cloner\VarCloner;
use Symfony\Component\VarDumper\Dumper\CliDumper;
use Symfony\Component\VarDumper\Dumper\ServerDumper;
use Symfony\Component\VarDumper\VarDumper;

class LanguageServerCommand extends Command
{
    protected $signature = 'language-server';

    protected $description = 'Start the language server.';

    public function handle(Configuration $configuration)
    {
        $logger = new NullLogger();

        LanguageServerBuilder::create(new LanguageServerDispatcherFactory($logger))
            ->build()
            ->run();
    }
}
