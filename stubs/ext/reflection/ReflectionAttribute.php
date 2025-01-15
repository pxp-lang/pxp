<?php 

class ReflectionAttribute
{
    public function getName(): string
    {
    }
    public function getTarget(): int
    {
    }
    public function isRepeated(): bool
    {
    }
    public function getArguments(): array
    {
    }
    public function newInstance(): object
    {
    }
    #[\Since('8.1')]
    public function __toString(): string
    {
    }
    private function __clone(): void
    {
    }
    private function __construct()
    {
    }
    /**
     * @var int
     * @cvalue REFLECTION_ATTRIBUTE_IS_INSTANCEOF
     */
    #[\Since('8.2')]
    #[\Until('8.4')]
    public const IS_INSTANCEOF = UNKNOWN;
    /** @cvalue REFLECTION_ATTRIBUTE_IS_INSTANCEOF */
    #[\Since('8.4')]
    public const int IS_INSTANCEOF = UNKNOWN;
}