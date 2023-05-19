<?php

namespace Pxp\Common\Configuration;

final class TranspilerOptions
{
    public function __construct(
        public readonly bool $strictTypes = false,
        public readonly bool $sourceMap = false,
    ) {}
}
