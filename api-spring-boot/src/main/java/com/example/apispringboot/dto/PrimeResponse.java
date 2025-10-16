package com.example.apispringboot.dto;

public class PrimeResponse {
    private boolean is_prime;

    public PrimeResponse() {}

    public PrimeResponse(boolean is_prime) {
        this.is_prime = is_prime;
    }

    public boolean isIs_prime() {
        return is_prime;
    }

    public void setIs_prime(boolean is_prime) {
        this.is_prime = is_prime;
    }
}
