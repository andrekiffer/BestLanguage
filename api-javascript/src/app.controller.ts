import { Controller, Post } from '@nestjs/common';
import { AppService } from './app.service';
import type { AppDto } from './app.dto';

@Controller()
export class AppController {
  constructor(private readonly appService: AppService) {}

  @Post()
  isPrime(data: {number: number}): AppDto {
    return { is_prime: this.appService.isPrime(data.number) };
  }
}
