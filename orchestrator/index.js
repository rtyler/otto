/*
 * This file is just a simple demonstration service implementing a static
 * orchestrator response to generate a proof-of-concept agent
 *
 * The "real" orchestrator will be much different
 */

const express = require('express');
const uuidv4  = require('uuid/v4');

const app = express();


app.get('/', (req, res) => {
  res.send('Otto Orchestrator');
});

app.get('/v1/manifest/:agentIdent', (req, res) => {
  const ident = req.params.agentIdent;
  const context_id = uuidv4();

  res.json(
{
  "self" : ident,
  "services" : {
    "orchestrator" : "http://localhost:3030/",
    "datastore"    : "http://localhost:3031/",
    "objectstore"  : "http://localhost:3031/",
    "eventbus"     : "http://localhost:3040/"
  },
  "ops" : [
    {
      "op_id"    : context_id,
      "type"     : "BEGINCTX",
      "data" : {
        "name"          : "Build",
        "context_type"  : "stage",
        "parallel"      : false
      }
    },
    {
      "op_id"    : "uuid2",
      "type"     : "RUNPROC",
      "data" : {
        "script"    : "echo \"Hello World\"",
        "env"       : {},
        "timeout_s" : "600"
      }
    },
    {
      "op_id"    : context_id,
      "type"     : "ENDCTX",
      "data" : {
        "name"          : "Build",
        "context_type"  : "stage",
        "parallel"      : false
      }
    }
  ]
}
  );
});


app.listen(3030, () => console.log('Listening on localhost:3030'));
