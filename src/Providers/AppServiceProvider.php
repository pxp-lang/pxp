<?php

namespace Pxp\Providers;

use Pxp\Common\Configuration\Configuration;
use Pxp\Transpiler\Parser;
use Pxp\Transpiler\Transpiler;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Support\ServiceProvider;

class AppServiceProvider extends ServiceProvider
{
    /**
     * Bootstrap any application services.
     */
    public function boot(): void
    {
        Model::unguard();
    }

    /**
     * Register any application services.
     */
    public function register(): void
    {
        $this->app->singleton(Configuration::class, function () {
            return Configuration::fromArray(
                file_exists(getcwd() . '/pxp.json')
                    ? json_decode(file_get_contents(getcwd() . '/pxp.json'), associative: true)
                    : []
            );
        });

        $this->app->singleton(Parser::class);
        $this->app->singleton(Transpiler::class);
    }
}
