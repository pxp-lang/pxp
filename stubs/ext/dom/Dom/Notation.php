<?php 

namespace Dom;

#[\Since('8.4')]
class Notation extends \Dom\Node
{
    /**
     * @readonly
     * @virtual
     */
    public string $publicId;
    /**
     * @readonly
     * @virtual
     */
    public string $systemId;
}