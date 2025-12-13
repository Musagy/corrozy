<?php
declare(strict_types=1);

namespace MyApp\Variable;

/** @var int $edad */
$edad = 25;
/** @var float $precio */
$precio = 19.99;
/** @var string $nombre */
$nombre = "Juan";
/** @var bool $activo */
$activo = true;
$cualquier_cosa = "dinámico";
$numero_magico = 42;
$pi_aproximado = 3.14159;
$interpolated_string = "Hello $nombre";
$raw_string = 'Hello $nombre';
$mixed_content = "Price: $precio USD";
/** @var int */
const MAX_USERS = 1000;
const APP_NAME = "Mi Aplicación";
const VERSION = '1.0.0';
echo $edad . "\n";
echo $precio . "\n";
echo $nombre . "\n";
echo $activo . "\n";
echo $cualquier_cosa . "\n";
echo $interpolated_string . "\n";
echo $raw_string . "\n";
echo $mixed_content . "\n";
echo "Constants:" . "\n";
echo $MAX_USERS . "\n";
echo $APP_NAME . "\n";
echo $VERSION . "\n";
$_special_name = "hola";
