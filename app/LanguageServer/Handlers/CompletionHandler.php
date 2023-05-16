<?php

namespace App\LanguageServer\Handlers;

use Amp\CancellationToken;
use Amp\Promise;
use Phpactor\LanguageServer\Core\Handler\CanRegisterCapabilities;
use Phpactor\LanguageServer\Core\Handler\Handler;
use Phpactor\LanguageServerProtocol\CompletionItem;
use Phpactor\LanguageServerProtocol\CompletionItemKind;
use Phpactor\LanguageServerProtocol\CompletionList;
use Phpactor\LanguageServerProtocol\CompletionOptions;
use Phpactor\LanguageServerProtocol\CompletionParams;
use Phpactor\LanguageServerProtocol\ServerCapabilities;

use function Amp\call;

class CompletionHandler implements Handler, CanRegisterCapabilities
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
            return new CompletionList(false, [
                new CompletionItem('Foobar'),
                new CompletionItem('$foo', kind: CompletionItemKind::VARIABLE)
            ]);
        });
    }

    public function registerCapabiltiies(ServerCapabilities $capabilities): void
    {
        $capabilities->completionProvider = new CompletionOptions([
            ':',
            '>',
            '$',
            '(',
        ]);

        $capabilities->completionProvider->resolveProvider = false;
    }
}
