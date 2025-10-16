<?php

use App\Http\Controllers\PrimeController;
use Illuminate\Support\Facades\Route;

Route::post('/', [PrimeController::class, 'checkPrime']);
