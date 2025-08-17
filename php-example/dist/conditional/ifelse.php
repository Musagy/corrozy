<?php
declare(strict_types=1);

namespace MyApp\Conditional;

$rand_num = random_int(1, 3);
if ($rand_num == 1) {
    echo "Condition is true" . "\n";
} elseif ($rand_num == 2) {
    echo "Condition is false" . "\n";
} else {
    echo "Condition is neither true nor false" . "\n";
}