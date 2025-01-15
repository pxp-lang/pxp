<?php 

#[\Since('8.1')]
class ReflectionEnum extends \ReflectionClass
{
    #[\Until('8.2')]
    public function getBackingType(): ?ReflectionType
    {
    }
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
    #[\Since('8.2')]
    public function getBackingType(): ?ReflectionNamedType
    {
    }
}