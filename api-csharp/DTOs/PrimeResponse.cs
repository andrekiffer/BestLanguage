using System.Text.Json.Serialization;

namespace ApiCsharp.DTOs
{
    public class PrimeResponse
    {
        [JsonPropertyName("is_prime")]
        public bool IsPrime { get; set; }

        public PrimeResponse(bool isPrime)
        {
            IsPrime = isPrime;
        }
    }
}
