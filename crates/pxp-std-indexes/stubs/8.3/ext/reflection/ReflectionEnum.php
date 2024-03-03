<?php

class ReflectionEnum extends \ReflectionClass
{
    public function __construct(object|string $objectOrClass)
    {
    }
    public function hasCase(string $name): bool
    {
    }
    public function getCase(string $name): ReflectionEnumUnitCase
    {
    }
    public function getCases(): array
    {
    }
    public function isBacked(): bool
    {
    }
    public function getBackingType(): ?ReflectionNamedType
    {
    }
}