require('module-alias/register');

import logger from '@otto/logger';
import { serveApp } from '@otto/server'

import feathers from '@feathersjs/feathers';
import configuration from '@feathersjs/configuration';
import express from '@feathersjs/express';

logger.info('Starting orchestrator');

const app = express(feathers());

// We need to fish out the orchestrator specific settings
const settings = configuration()(app);
app.configure((app) => {
  Object.keys(settings['orchestrator']).forEach((key) => {
    app.set(key, settings['orchestrator'][key]);
  });
});


app.configure(express.rest());


app.configure((app : feathers.Application) => {
  const service = {
    get: (id : feathers.Id) => {
      logger.info(`Invoking get for ${id}`);
      const response = {
        self: id,
        services: {
          datastore: 'http://localhost:3031/',
        },
        ops: [
          {
            id: '0xdeadbeef',
            context: '0x1',
            type: 'RUNPROC',
            data: {
              script: 'echo "Hello World"',
              env: {},
              timeout_s: 600,
            },
          },
        ],
      };
      return Promise.resolve(response);
    },
  };
  app.use('/v1/manifest', service);
});

serveApp(app);
