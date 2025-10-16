package com.example.apispringboot.service;

import org.springframework.stereotype.Service;

@Service
public class PrimeService {
    
    /**
     * Verifica se um número é primo
     */
    public boolean isPrime(int number) {
        if (number < 2) {
            return false;
        }
        
        for (int i = 2; i <= Math.sqrt(number); i++) {
            if (number % i == 0) {
                return false;
            }
        }
        
        return true;
    }
}
