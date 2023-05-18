<?php

namespace App\Indexer;

use App\LanguageServer\TolerantParser;

final class CachingParser
{
    private array $cache = [];

    public function __construct(
        private TolerantParser $parser,
    ) {}

    public function parse(string $code): array
    {
        $hash = md5($code);

        return $this->cache[$hash] ??= $this->parser->parse($code);
    }
}
