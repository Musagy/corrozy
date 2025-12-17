<?php
declare(strict_types=1);

namespace MyApp\Literal;

$interpolated_string = "Hello $nombre";
$raw_string = 'Hello $nombre';
$mixed_content = "Price: $precio USD";
echo $interpolated_string . "\n";
echo $raw_string . "\n";
echo $mixed_content . "\n";
