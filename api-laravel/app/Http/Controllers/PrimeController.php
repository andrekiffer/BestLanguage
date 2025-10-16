<?php

namespace App\Http\Controllers;

use App\Http\Requests\CheckPrimeRequest;
use App\Services\PrimeService;
use Illuminate\Http\JsonResponse;

class PrimeController extends Controller
{
    public function __construct(
        private PrimeService $primeService
    ) {}

    /**
     * Check if a number is prime
     */
    public function checkPrime(CheckPrimeRequest $request): JsonResponse
    {
        $number = $request->validated()['number'];
        $isPrime = $this->primeService->isPrime($number);

        return response()->json([
            'is_prime' => $isPrime
        ]);
    }
}
