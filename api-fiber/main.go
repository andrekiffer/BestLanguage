package main

import (
	"math"

	"github.com/gofiber/fiber/v2"
)

func main() {
	app := fiber.New()

	app.Get("/", func(c *fiber.Ctx) error {
		return c.SendString("Hello, World!")
	})

	app.Post("/", func(c *fiber.Ctx) error {
		var body map[string]int
		if err := c.BodyParser(&body); err != nil {
			return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
				"error": "Invalid request body",
			})
		}
		return c.JSON(fiber.Map{
			"is_prime": isPrime(body["number"]),
		})
	})

	app.Listen(":3000")
}

func isPrime(number int) bool {
	if number < 2 {
		return false
	}
	for i := 2; i <= int(math.Sqrt(float64(number))); i++ {
		if number%i == 0 {
			return false
		}
	}
	return true
}
