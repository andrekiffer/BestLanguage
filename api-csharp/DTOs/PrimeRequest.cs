using System.ComponentModel.DataAnnotations;
using System.Text.Json.Serialization;

namespace ApiCsharp.DTOs
{
    public class PrimeRequest
    {
        [Required(ErrorMessage = "Number is required")]
        [Range(0, int.MaxValue, ErrorMessage = "Number must be at least 0")]
        [JsonPropertyName("number")]
        public int Number { get; set; }
    }
}
