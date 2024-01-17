import pino, { type Logger } from 'pino'

export class LoggerInitializer {
  public init (): Logger {
    return pino({
      // eslint-disable-next-line @typescript-eslint/strict-boolean-expressions
      level: process.env.LOG_LEVEL || 'debug'
    })
  }
}
