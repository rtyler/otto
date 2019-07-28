/*
 * The ParseListener is the initial entrypoint for building the graph
 */
import * as otto from '@otto/grammar/Otto'
import { OttoLexer } from '@otto/grammar/OttoLexer'
import { OttoListener } from '@otto/grammar/OttoListener'
import logger from '@otto/logger'

import { Orf, EMPTY_ORF } from '@otto-parser/Orf'
import Runtime from '@otto-parser/Runtime'
import Stage from '@otto-parser/Stage'

export default class ParseListener extends OttoListener {
  /**
   * The orf is generated per instance of the ParseListener, which means that
   * the object should not be re-used for multiple parse runs
   */
  protected readonly orf: Orf
  protected completed: Boolean = false
  protected currentStage: Stage | null

  constructor() {
    super()
    this.orf = new Orf()
  }

  public enterStage(ctx) {
    logger.debug('Parsing stage at line %s:%s', ctx.start.line, ctx.start.column)
    this.currentStage = new Stage()
  }

  public exitStage(ctx) {
    if (this.currentStage === null) {
      throw new Error("How was it possible to exit a stage without entering?")
      return
    }

    ctx.children.filter(c => this.withoutTerminals(c))
      .forEach((child) => {
        /*
         * We can have many different children based on the grammar
         * and we need to pick out their children and assign settings,
         * etc
         */

        child.children.filter(c => this.withoutTerminals(c))
          .forEach((c) => {
            switch (c.constructor.name) {
              case 'StepsContext':
                break;
              case 'SettingsContext':
                // At the moment we only care about one setting
                let [key, value] = this.parseSettingsContext(c)
                if (key == 'name') {
                  this.currentStage!.name = value
                }
                break;
            }
          })
      })

    /*
     * For now only bothering to add the stage if it has a name
     *
     * Will need a cleaner way to do stage structure validation and error
     * handling in the future
     */
    if (this.currentStage.name) {
      this.orf.addStage(this.currentStage)
    }
    this.currentStage = null
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
        settings = this.parseSettingBlock(child)
    })

    const runtime = new Runtime(runtimeType, settings)
    this.orf.addRuntime(runtime)
    if (this.currentStage != null) {
      this.currentStage.runtime = runtime
    }
  }

  protected parseSettingsContext(settingsContext) {
    const setting = settingsContext.getChild(0)
    const key = setting.ID().getText()
    const value = setting.StringLiteral().getText()
    /* Slicing to remove the leading and trailing quote characters
      * from the string
      */
    return [key, value.slice(1, -1)]
  }

  protected parseSettingBlock(settingBlockContext) {
    const settings = {}
    settingBlockContext.children
      .filter(c => this.withoutTerminals(c))
      .map((setting) => {
        let [key, value] = this.parseSettingsContext(setting)
        settings[key] = value
      })
    return settings
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
