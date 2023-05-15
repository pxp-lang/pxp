<?php

namespace App\Common\Configuration;

final class Configuration
{
    public function __construct(
        public readonly array $paths,
        public readonly TranspilerOptions $transpiler,
    ) {}

    public static function fromArray(array $config): self
    {
        return new self(
            paths: $config['paths'] ?? [],
            transpiler: new TranspilerOptions(
                strictTypes: $config['transpiler']['strictTypes'] ?? false,
                sourceMap: $config['transpiler']['sourceMap'] ?? false,
            )
        );
    }
}
