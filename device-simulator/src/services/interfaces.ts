export interface IRandomGenerator {
  generate: () => number
}

export interface DeviceData {
  device: string
  value: string
  type: 'TEMP' | 'HUMIDITY'
}

export interface IMessaging {
  pub: (data: DeviceData) => void
}
