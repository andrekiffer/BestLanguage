namespace ApiCsharp.Services
{
    public class PrimeService
    {
        /// <summary>
        /// Verifica se um número é primo
        /// </summary>
        /// <param name="number">Número a ser verificado</param>
        /// <returns>True se o número for primo, False caso contrário</returns>
        public bool IsPrime(int number)
        {
            if (number < 2)
            {
                return false;
            }

            for (int i = 2; i <= Math.Sqrt(number); i++)
            {
                if (number % i == 0)
                {
                    return false;
                }
            }

            return true;
        }
    }
}
