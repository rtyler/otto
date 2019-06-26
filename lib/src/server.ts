/**
 * This module acts as the simple wrapper for serving any feathers application
 * in the project
 */

import logger from '@otto/logger';

import { Application } from '@feathersjs/express';

/**
 * The serveApp function expects a Feathers application which it can start
 * serving on its configured port
 *
 * This method is *asynchronous* and will not return a success or fail
 *
 * @param app An instantiated feathers application
 */
export function serveApp(app: Application) {
  const port: Number = app.get('port');
  const server: any = app.listen(port);

  process.on('unhandledRejection', (reason, p) =>
    logger.error('Unhandled Rejection at: Promise ', p, reason)
  );

  server.on('listening', () =>
    logger.info('Feathers application started on http://%s:%d', app.get('host'), port)
  );
}
