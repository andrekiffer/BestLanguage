package com.example.apispringboot.controller;

import com.example.apispringboot.dto.PrimeRequest;
import com.example.apispringboot.dto.PrimeResponse;
import com.example.apispringboot.service.PrimeService;
import jakarta.validation.Valid;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

@RestController
public class PrimeController {
    
    @Autowired
    private PrimeService primeService;
    
    /**
     * Verifica se um número é primo
     */
    @PostMapping("/")
    public ResponseEntity<PrimeResponse> checkPrime(@Valid @RequestBody PrimeRequest request) {
        boolean isPrime = primeService.isPrime(request.getNumber());
        return ResponseEntity.ok(new PrimeResponse(isPrime));
    }
}
