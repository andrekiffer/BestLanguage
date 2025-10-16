using Microsoft.AspNetCore.Mvc;
using ApiCsharp.DTOs;
using ApiCsharp.Services;

namespace ApiCsharp.Controllers
{
    [ApiController]
    public class PrimeController : ControllerBase
    {
        private readonly PrimeService _primeService;

        public PrimeController(PrimeService primeService)
        {
            _primeService = primeService;
        }

        /// <summary>
        /// Verifica se um número é primo
        /// </summary>
        /// <param name="request">Objeto contendo o número a ser verificado</param>
        /// <returns>Resultado indicando se o número é primo</returns>
        [HttpPost("/")]
        public ActionResult<PrimeResponse> CheckPrime([FromBody] PrimeRequest request)
        {
            if (!ModelState.IsValid)
            {
                return BadRequest(ModelState);
            }

            bool isPrime = _primeService.IsPrime(request.Number);
            return Ok(new PrimeResponse(isPrime));
        }
    }
}
