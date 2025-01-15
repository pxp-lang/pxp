<?php 

namespace Dom;

#[\Since('8.4')]
interface ParentNode
{
    public function append(Node|string ...$nodes): void;
    public function prepend(Node|string ...$nodes): void;
    public function replaceChildren(Node|string ...$nodes): void;
    public function querySelector(string $selectors): ?Element;
    public function querySelectorAll(string $selectors): NodeList;
}