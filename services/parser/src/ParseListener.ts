/*
 * The ParseListener is the initial entrypoint for building the graph
 */
require('module-alias/register')

import { OttoLexer } from '@otto/grammar/OttoLexer'
import { OttoListener } from '@otto/grammar/OttoListener'
import { Otto } from '@otto/grammar/Otto'

export default class ParseListener extends OttoListener {
}
