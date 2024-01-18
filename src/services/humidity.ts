import { type IRandomGenerator, type IMessaging, type DeviceData } from './interfaces'

export class HumidityGeneratorService {
  private readonly HUMIDITY_DATA_INTERVAL: number

  constructor (
    private readonly randomGenerator: IRandomGenerator,
    private readonly messaging: IMessaging
  ) {
    const dataInterval = process.env.HUMIDITY_DATA_INTERVAL

    if (dataInterval === undefined || dataInterval === null) {
      throw new Error('invalid humidity data interval')
    }

    this.HUMIDITY_DATA_INTERVAL = Number(dataInterval)
  }

  public do () {
    setInterval(() => {
      const random = this.randomGenerator.generate()

      const data: DeviceData = {
        device: 'random',
        type: 'HUMIDITY',
        value: String(random)
      }

      this.messaging.pub(data)
    }, this.HUMIDITY_DATA_INTERVAL)
  }
}
