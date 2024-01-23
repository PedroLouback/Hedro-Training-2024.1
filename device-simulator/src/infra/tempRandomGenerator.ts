import { type Logger } from 'pino'
import { type IRandomGenerator } from '../services/interfaces'

function random (min: number, max: number): number {
  return Math.floor(min + Math.random() * (max - min))
}

export class TempRandomGenerator implements IRandomGenerator {
  private readonly TEMP_MIN = 10
  private readonly TEMP_MAX = 15

  constructor (private readonly logger: Logger) {}

  generate (): number {
    this.logger.debug('generated random data')
    const randomValue = random(this.TEMP_MIN, this.TEMP_MAX)
    return randomValue
  }
}
