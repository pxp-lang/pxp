<?php 

namespace Dom;

#[\Since('8.4')]
interface ChildNode
{
    public function remove(): void;
    public function before(Node|string ...$nodes): void;
    public function after(Node|string ...$nodes): void;
    public function replaceWith(Node|string ...$nodes): void;
}