<?php

namespace App\LanguageServer\Handlers;

use Amp\CancellationToken;
use Amp\Promise;
use Phpactor\LanguageServer\Core\Handler\Handler;
use Phpactor\LanguageServerProtocol\CompletionItem;
use Phpactor\LanguageServerProtocol\CompletionParams;

use function Amp\call;

class CompletionHandler implements Handler
{
    public function methods(): array
    {
        return [
            'textDocument/completion' => 'complete',
        ];
    }

    public function complete(CompletionParams $params, CancellationToken $cancellationToken): Promise
    {
        return call(function () {
            return [
                new CompletionItem('Foobar!')
            ];
        });
    }
}
