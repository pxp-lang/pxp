<?php

namespace Pxp\Commands;

use Pxp\Common\Configuration\Configuration;
use Illuminate\Console\Scheduling\Schedule;
use LaravelZero\Framework\Commands\Command;

class AnalyseCommand extends Command
{
    protected $signature = 'analyse';

    protected $description = 'Statically analyse your PXP and PHP code, together.';

    public function handle(Configuration $configuration)
    {
        $this->warn('Static analysis is still a work-in-progress and might not produce accurate results!');
    }
}
