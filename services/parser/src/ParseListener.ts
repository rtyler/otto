/*
 * The ParseListener is the initial entrypoint for building the graph
 */
import { Otto } from '@otto/grammar/Otto'
import { OttoLexer } from '@otto/grammar/OttoLexer'
import { OttoListener } from '@otto/grammar/OttoListener'
import logger from '@otto/logger'

import { Orf, EMPTY_ORF } from '@otto-parser/Orf'

export default class ParseListener extends OttoListener {
  /**
   * The orf is generated per instance of the ParseListener, which means that
   * the object should not be re-used for multiple parse runs
   */
  protected readonly orf: Orf
  protected completed: Boolean = false

  constructor() {
    super()
    this.orf = new Orf()
  }

  public enterStages(ctx) {
    logger.debug('Parsing stage at line %s:%s', ctx.start.line, ctx.start.column)
  }

  public exitPipeline(ctx) {
    logger.debug('Exiting pipeline at line %s:%s', ctx.start.line, ctx.start.column)
    this.completed = true
  }

  /**
   * Return the computed orf only if the processing has been completed,
   * otherwise a null will be returned
   */
  public getOrf(): Orf {
    if (this.completed) {
      return this.orf
    }
    return EMPTY_ORF
  }
}
