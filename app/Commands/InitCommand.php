<?php

namespace Pxp\Commands;

use Illuminate\Console\Scheduling\Schedule;
use Illuminate\Support\Facades\File;
use LaravelZero\Framework\Commands\Command;

class InitCommand extends Command
{
    protected $signature = 'init';

    protected $description = 'Initialise your project.';

    public function handle()
    {
        $cwd = getcwd();

        if (file_exists($cwd . '/pxp.json')) {
            $this->components->error('Project already initialised.');

            return self::FAILURE;
        }

        $stub = file_get_contents(__DIR__ . '/stubs/pxp.json');

        File::put($cwd . '/pxp.json', $stub);

        $this->components->info('Project initialised.');

        return self::SUCCESS;
    }
}
