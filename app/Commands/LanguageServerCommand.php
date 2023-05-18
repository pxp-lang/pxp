<?php

namespace App\Commands;

use App\Common\Configuration\Configuration;
use App\LanguageServer\LanguageServerDispatcherFactory;
use Exception;
use Illuminate\Console\Scheduling\Schedule;
use Illuminate\Database\Migrations\MigrationRepositoryInterface;
use LaravelZero\Framework\Commands\Command;
use LaravelZero\Framework\Components\Database\Migrator;
use Phpactor\LanguageServer\LanguageServerBuilder;
use Psr\Log\NullLogger;

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
