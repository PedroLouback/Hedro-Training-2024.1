import { HumidityRandomGenerator } from './infra/humidityRandomGenerator'
import { LoggerInitializer } from './infra/logger'
import { Messaging } from './infra/messaging'
import { TempRandomGenerator } from './infra/tempRandomGenerator'
import { HumidityGeneratorService } from './services/humidity'
import { type IMessaging, type IRandomGenerator } from './services/interfaces'
import { TempGeneratorService } from './services/temp'
import dotenv from 'dotenv'

function main () {
  const { logger, tempRandom, humidityRandom, messaging } = setup()

  startup(tempRandom, humidityRandom, messaging)

  logger.info('application running')
}

function setup () {
  dotenv.config()

  const logger = new LoggerInitializer().init()

  logger.info('starting application..')

  const messaging = new Messaging(logger)
  messaging.connect()
  const tempRandom = new TempRandomGenerator(logger)
  const humidityRandom = new HumidityRandomGenerator(logger)

  return {
    logger,
    messaging,
    tempRandom,
    humidityRandom
  }
}

function startup (tempRandom: IRandomGenerator, humidityRandom: IRandomGenerator, messaging: IMessaging) {
  const tempGeneratorService = new TempGeneratorService(tempRandom, messaging)
  const humidityRandomService = new HumidityGeneratorService(humidityRandom, messaging)

  tempGeneratorService.do()
  humidityRandomService.do()
}

main()
