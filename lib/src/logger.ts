/**
 * The logger package will create a simple default logger for use with feathers
 * services
 */

import { createLogger, format, transports } from 'winston'

export default createLogger({
  // To see more detailed errors, change this to 'debug'
  format: format.combine(
    format.splat(),
    format.simple(),
  ),
  level: process.env.LOG_LEVEL || 'info',
  transports: [
    new transports.Console(),
  ],
})
