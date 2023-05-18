<?php

namespace App\Commands;

use App\Common\Configuration\Configuration;
use App\Indexer\Indexer;
use App\Indexer\Repl\Repl;
use Illuminate\Console\Scheduling\Schedule;
use LaravelZero\Framework\Commands\Command;

class IndexCommand extends Command
{
    protected $signature = 'index';

    protected $description = 'Index the current directory and query the index via a REPL.';

    public function handle(Configuration $configuration, Indexer $indexer)
    {
        $start = microtime(true);

        $this->info('Generating index...');

        $index = $indexer->index([getcwd()]);

        $this->info('Index generated.');
        $this->warn('Indexing took ' . microtime(true) - $start . ' seconds.');

        readline_completion_function(function (string $input) {
            return collect([
                'reindex',
                'find',
            ])
                ->filter(fn (string $suggestion) => str_starts_with($suggestion, $input))
                ->all();
        });

        while (true) {
            $command = readline('> ');

            if ($command === false) {
                break;
            }

            if (! $command) {
                continue;
            }

            readline_add_history($command);

            $command = trim($command);

            [$command, $arguments] = count(explode(' ', $command, 2)) > 1 ? explode(' ', $command, 2) : [$command, ''];
            $arguments = explode(' ', $arguments);

            if ($command === 'reindex') {
                $start = microtime(true);

                $index = $indexer->index([getcwd()]);

                $this->warn('Indexing took ' . microtime(true) - $start . ' seconds.');
            }

            if ($command === 'find') {
                if (count($arguments) === 0) {
                    $this->error('The "find" command requires at least 1 argument, i.e. `find \sprintf`');
                }

                // Strip quotations from a literal string argument.
                $entity = trim($arguments[0], '"');

                if ($function = $index->function($entity)) {
                    dump($function);
                } else {
                    $this->warn('Unable to locate entity with name ' . $entity . '.');
                }
            }

            if ($command === 'exit' || $command === 'quit') {
                break;
            }
        }
    }
}
