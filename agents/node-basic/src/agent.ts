/*
 * The agent module is the main entry point for the TypeScript Otto agent
 */

import child_process from 'child_process';
import fs from 'fs';

import feathers from '@feathersjs/feathers';
import rest from '@feathersjs/rest-client';
import request from 'request';
import tmp from 'tmp';

import logger from './logger';

logger.info('Agent starting up..');

const app = feathers();
const requestClient = request.defaults();
const restClient = rest('http://localhost:3030');

app.configure(restClient.request(requestClient));

app.service('/v1/manifest').get('agentname').then((manifest) => {
  const promises = manifest.ops.map((operation) => {
    if (operation.type === 'RUNPROC') {
      return new Promise((resolve, reject) => {
        logger.info(`I have been instructed to run: ${operation.data.script}`);
        const tmpFile = tmp.fileSync();
        fs.writeFileSync(tmpFile.fd, '#!/bin/sh\n', { encoding: 'utf8' });
        fs.writeFileSync(tmpFile.fd, operation.data.script, { encoding: 'utf8' });
        logger.debug(`Wrote command to: ${tmpFile.name}`);
        const sub = child_process.spawn('sh', [tmpFile.name]);
        sub.stdout.on('data', data => logger.info(data));
        sub.stderr.on('data', data => logger.error(data));
        sub.on('close', rc => resolve(rc));
        sub.on('error', e => reject(e));
      });
    } else {
      return Promise.resolve(null);
    }
  });
  // Execution is not chained at the moment
  return Promise.all(promises);
});
