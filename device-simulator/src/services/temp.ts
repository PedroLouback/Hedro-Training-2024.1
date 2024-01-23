import { type IRandomGenerator, type IMessaging, type DeviceData } from './interfaces'

export class TempGeneratorService {
  private readonly TEMP_DATA_INTERVAL: number

  constructor (
    private readonly randomGenerator: IRandomGenerator,
    private readonly messaging: IMessaging
  ) {
    const tempDataInterval = process.env.TEMP_DATA_INTERVAL

    if (tempDataInterval === undefined || tempDataInterval === null) {
      throw new Error('invalid temperature data interval')
    }

    this.TEMP_DATA_INTERVAL = Number(tempDataInterval)
  }

  public do () {
    setInterval(() => {
      const random = this.randomGenerator.generate()

      const data: DeviceData = {
        device: 'random',
        type: 'TEMP',
        value: String(random)
      }

      this.messaging.pub(data)
    }, this.TEMP_DATA_INTERVAL)
  }
}
