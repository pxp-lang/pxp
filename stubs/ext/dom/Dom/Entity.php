<?php 

namespace Dom;

#[\Since('8.4')]
class Entity extends \Dom\Node
{
    /**
     * @readonly
     * @virtual
     */
    public ?string $publicId;
    /**
     * @readonly
     * @virtual
     */
    public ?string $systemId;
    /**
     * @readonly
     * @virtual
     */
    public ?string $notationName;
}