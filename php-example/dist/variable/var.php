<?php
declare(strict_types=1);

namespace MyApp\Variable;

/** @var int $edad */
$edad = 23;
/** @var float $precio */
$precio = 19.99;
/** @var string $nombre */
$nombre = "Diego";
/** @var bool $activo */
$activo = true;
$cualquier_cosa = "dinámico";
$numero_magico = 42;
$pi_aproximado = 3.14159;
$interpolated_string = "Hello $nombre";
$raw_string = 'Hello $nombre';
$mixed_content = "Price: $precio USD";
echo $edad . "\n";
echo $precio . "\n";
echo $nombre . "\n";
echo $activo . "\n";
echo $cualquier_cosa . "\n";
echo $interpolated_string . "\n";
echo $raw_string . "\n";
echo $mixed_content . "\n";
$_special_name = "hola";
