import antlr from 'antlr4'

import ParseListener from '@otto-parser/ParseListener'

import * as otto from '@otto/grammar/Otto'
import { OttoLexer } from '@otto/grammar/OttoLexer'
import { OttoListener } from '@otto/grammar/OttoListener'

import { EMPTY_ORF } from '@otto-parser/Orf'

function walkTreeFor(listener: ParseListener, input: string) {
  let chars = new antlr.InputStream(input)
  let lexer = new OttoLexer(chars)
  let tokens = new antlr.CommonTokenStream(lexer)
  let parser = new otto.Otto(tokens)
  parser.buildParseTrees = true
  let tree = parser.pipeline()
  antlr.tree.ParseTreeWalker.DEFAULT.walk(listener, tree)
  return true
}

describe('ParseListener', () => {
  describe('when parsing the most minimal valid pipeline', () => {
    it('should return an empty orf', () => {
      const p = 'pipeline { stages { stage { } } }'
      const l = new ParseListener()
      expect(walkTreeFor(l, p)).toBeTruthy()

      const orf = l.getOrf()
      expect(orf).toStrictEqual(EMPTY_ORF)
    })
  })

  describe('when parsing a simple pipeline', () => {
    function parsedOrf() {
      const p = `
        pipeline {
          stages {
            stage {
              name = 'Build'
              runtime {
                docker { image = 'alpine' }
              }
              steps {
                sh 'env'
              }
            }
          }
        }
      `
      const l = new ParseListener()
      walkTreeFor(l, p)
      return l.getOrf()
    }

    it('should return an orf', () => {
      const orf = parsedOrf()
      expect(orf).not.toStrictEqual(EMPTY_ORF)
    });

    it('the orf should have a single runtime', () => {
      const orf = parsedOrf()
      expect(orf.runtimes.length).toEqual(1)
      expect(orf.runtimes[0].runtimeType).toEqual('docker')
    })

    it('should parse out the stages', () => {
      const orf = parsedOrf()
      expect(orf.stages.length).toEqual(1)
      expect(orf.stages[0].name).toEqual('Build')
    })
  });
})
