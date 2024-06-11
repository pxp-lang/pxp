<?php

use App\Support\HasColor;

enum Role {
    case Admin;
    case User;
}

enum Status: int {
    case Active = 1;
    case Inactive = 0;
}

enum Color: string implements HasColor {
    case Red = 'red';
    case Green = 'green';
    case Blue = 'blue';

    public function getHex(): string {

    }
}