<?php

namespace Pxp;

use Pxp\Autoloader;

if (defined('PXP_AUTOLOAD_LOADED') && PXP_AUTOLOAD_LOADED === true) {
    return;
}

$autoloader = new Autoloader();
spl_autoload_register($autoloader->autoload(...), prepend: true);

define('PXP_AUTOLOAD_LOADED', true);
