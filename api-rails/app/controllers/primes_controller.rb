class PrimesController < ApplicationController
  def is_prime
    number = params[:number].to_i
    
    if number <= 0
      render json: { error: "Number must be a positive integer" }, status: :bad_request
      return
    end
    
    is_prime = PrimesService.is_prime(number)
    
    render json: { is_prime: is_prime }
  end
end
