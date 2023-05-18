<?php

namespace App\Common;

final class File
{
    public function __construct(
        public readonly string $path,
    ) {}
}
