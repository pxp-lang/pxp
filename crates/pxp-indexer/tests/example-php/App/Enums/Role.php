<?php

namespace App\Enums;

use Nested\Interfaces\Example;

enum Role: string implements Example
{
    case Admin = 'admin';
    case User = 'user';

    public function isAdmin(): bool
    {
        return $this->value === self::Admin;
    }

    public function isUser(): bool
    {
        return $this->value === self::User;
    }
}
