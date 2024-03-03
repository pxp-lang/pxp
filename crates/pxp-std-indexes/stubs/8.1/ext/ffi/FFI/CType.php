<?php

namespace FFI;

final class CType
{
    public function getName(): string
    {
    }
    public function getKind(): int
    {
    }
    public function getSize(): int
    {
    }
    public function getAlignment(): int
    {
    }
    public function getAttributes(): int
    {
    }
    public function getEnumKind(): int
    {
    }
    public function getArrayElementType(): CType
    {
    }
    public function getArrayLength(): int
    {
    }
    public function getPointerType(): CType
    {
    }
    public function getStructFieldNames(): array
    {
    }
    public function getStructFieldOffset(string $name): int
    {
    }
    public function getStructFieldType(string $name): CType
    {
    }
    public function getFuncABI(): int
    {
    }
    public function getFuncReturnType(): CType
    {
    }
    public function getFuncParameterCount(): int
    {
    }
    public function getFuncParameterType(int $index): CType
    {
    }
}