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

serveApp(app);
