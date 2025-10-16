<?php

namespace App\Services;

class PrimeService
{
    /**
     * Check if a number is prime
     */
    public function isPrime(int $number): bool
    {
        if ($number < 2) {
            return false;
        }

        for ($i = 2; $i <= sqrt($number); $i++) {
            if ($number % $i === 0) {
                return false;
            }
        }

        return true;
    }
}
