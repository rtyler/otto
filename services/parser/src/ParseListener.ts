/*
 * The ParseListener is the initial entrypoint for building the graph
 */
import * as otto from '@otto/grammar/Otto'
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
  protected runtimes = []

  constructor() {
    super()
    this.orf = new Orf()
  }

  public enterStages(ctx) {
    logger.debug('Parsing stage at line %s:%s', ctx.start.line, ctx.start.column)
  }

  /**
   * Return true if the node name is not a terminal
   */
  private withoutTerminals(node) {
    return node.constructor.name != 'TerminalNodeImpl'
  }

  public exitRuntime(ctx) {
    logger.debug('exiting runtime',)
    ctx.children
      .filter(c => this.withoutTerminals(c))
      .forEach((child) => {
      console.log(child.constructor.name)
      console.log(Object.getPrototypeOf(child))

      child.children.filter(c => this.withoutTerminals(c))
        .forEach(nc => console.log(Object.getPrototypeOf(nc)))
    })
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
