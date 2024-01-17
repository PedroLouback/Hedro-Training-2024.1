import { type Logger } from 'pino'
import { type IRandomGenerator } from '../services/interfaces'

export class TempRandomGenerator implements IRandomGenerator {
  private readonly TEMP_MIN = 10
  private readonly TEMP_MAX = 15

  constructor (private readonly logger: Logger) {}

  generate (): number {
    this.logger.debug('generated random data')
    return 0
  }
}
