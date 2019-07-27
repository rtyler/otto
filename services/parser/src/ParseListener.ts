/*
 * The ParseListener is the initial entrypoint for building the graph
 */
import * as otto from '@otto/grammar/Otto'
import { OttoLexer } from '@otto/grammar/OttoLexer'
import { OttoListener } from '@otto/grammar/OttoListener'
import logger from '@otto/logger'

import { Orf, EMPTY_ORF } from '@otto-parser/Orf'
import Runtime from '@otto-parser/Runtime'

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

  /**
   * Return true if the node name is not a terminal
   */
  private withoutTerminals(node) {
    return node.constructor.name != 'TerminalNodeImpl'
  }

  public exitRuntime(ctx) {
    logger.debug('exiting runtime')

    let settings = {}
    let runtimeType

    ctx.children
      .filter(c => this.withoutTerminals(c))
      .forEach((child) => {
        runtimeType = child.ID().getText();
        /*
         * NOTE: the runtimeType has not yet been validated at this point. It is unclear whether
         * the runtime type should be validated for correctness at parse time. Or at least
         * this early in parse time
         */

        // Look at all the settings set in the block and do something with them
        child.children.filter(c => this.withoutTerminals(c))
          .forEach((setting) => {
            setting = setting.getChild(0)
            const key = setting.ID().getText()
            const value = setting.StringLiteral().getText()
            // Slicing to remove the leading and trailing quote characters from the string
            settings[key] = value.slice(1, -1)
          })
    })

    this.orf.addRuntime(new Runtime(runtimeType, settings))
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
