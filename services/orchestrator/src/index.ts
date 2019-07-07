import modalias from 'module-alias'
modalias()

import configuration from '@feathersjs/configuration'
import express from '@feathersjs/express'
import feathers from '@feathersjs/feathers'

import logger from '@otto/logger'
import { serveApp } from '@otto/server'

logger.info('Starting orchestrator')

const app = express(feathers())
// We need to fish out the orchestrator specific settings
const settings = configuration()(app)
app.configure(() => {
  // tslint:disable:no-string-literal
  Object.keys(settings['orchestrator']).forEach((key) => {
    app.set(key, settings['orchestrator'][key])
  })
  // tslint:enable:no-string-literal
})

app.configure(express.rest())

app.configure(() => {
  const service = {
    get: (id: feathers.Id) => {
      logger.info(`Invoking get for ${id}`)
      const response = {
        ops: [
          {
            context: '0x1',
            data: {
              env: {},
              script: 'echo "Hello World"',
              timeout_s: 600,
            },
            id: '0xdeadbeef',
            type: 'RUNPROC',
          },
        ],
        self: id,
        services: {
          datastore: 'http://localhost:3031/',
        },
      }
      return Promise.resolve(response)
    },
  }
  app.use('/v1/manifest', service)
})

serveApp(app)
