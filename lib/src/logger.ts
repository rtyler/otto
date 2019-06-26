/**
 * The logger package will create a simple default logger for use with feathers
 * services
 */

import { createLogger, format, transports } from 'winston';

export default createLogger({
  // To see more detailed errors, change this to 'debug'
  level: 'info',
  format: format.combine(
    format.splat(),
    format.simple()
  ),
  transports: [
    new transports.Console()
  ],
});
