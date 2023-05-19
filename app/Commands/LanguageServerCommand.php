<?php

namespace Pxp\Commands;

use Pxp\Common\Configuration\Configuration;
use Pxp\LanguageServer\LanguageServerDispatcherFactory;
use LaravelZero\Framework\Commands\Command;
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
