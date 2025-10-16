package com.example.apispringboot.dto;

import jakarta.validation.constraints.Min;
import jakarta.validation.constraints.NotNull;

public class PrimeRequest {
    @NotNull(message = "Number is required")
    @Min(value = 0, message = "Number must be at least 0")
    private Integer number;

    public PrimeRequest() {}

    public PrimeRequest(Integer number) {
        this.number = number;
    }

    public Integer getNumber() {
        return number;
    }

    public void setNumber(Integer number) {
        this.number = number;
    }
}
