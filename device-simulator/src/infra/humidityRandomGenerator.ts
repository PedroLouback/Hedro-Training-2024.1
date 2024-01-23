import { type Logger } from 'pino'
import { type IRandomGenerator } from '../services/interfaces'

function random (min: number, max: number): number {
  return (min + Math.random() * (max - min))
}

export class HumidityRandomGenerator implements IRandomGenerator {
  private readonly HUMIDITY_MIN = 0.3
  private readonly HUMIDITY_MAX = 0.8

  constructor (private readonly logger: Logger) {}

  generate (): number {
    this.logger.debug('generated random data')
    const randomValue = random(this.HUMIDITY_MIN, this.HUMIDITY_MAX)
    return Number(randomValue.toFixed(1))
  }
}
