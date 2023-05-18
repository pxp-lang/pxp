<?php

namespace App\Providers;

use App\Common\Configuration\Configuration;
use App\Common\DocblockParser;
use App\Indexer\CachingParser;
use App\Indexer\Structures\StructureRepository;
use App\LanguageServer\TolerantParser;
use App\Transpiler\Parser;
use App\Transpiler\Transpiler;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Support\ServiceProvider;
use PHPStan\PhpDocParser\Lexer\Lexer;
use PHPStan\PhpDocParser\Parser\ConstExprParser;
use PHPStan\PhpDocParser\Parser\PhpDocParser;
use PHPStan\PhpDocParser\Parser\TypeParser;

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
        $this->app->singleton(TolerantParser::class);
        $this->app->singleton(CachingParser::class);
        $this->app->singleton(Transpiler::class);
    }
}
