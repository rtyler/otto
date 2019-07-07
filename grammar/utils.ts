import antlr from 'antlr4';

import { ErrorListener } from 'antlr4/error/ErrorListener';
import { OttoLexer } from './build/parser/JavaScript/OttoLexer';
import { OttoListener } from './build/parser/JavaScript/OttoListener';
import { Otto } from './build/parser/JavaScript/Otto';

export const MIN_PIPELINE = 'pipeline { stages { stage { } } }';

class Visitor {
  visitChildren(ctx) {
    if (!ctx) {
      return;
    }

    if (ctx.children) {
      return ctx.children.map(child => {
        if (child.children && child.children.length != 0) {
          return child.accept(this);
        } else {
          return child.getText();
        }
      });
    }
  }
}

class JestListener extends ErrorListener {
  public errors: Array<any> = [];

  syntaxError(recognizer, offendingSymbol, line, column, msg, e) {
    this.errors.push({
      line: line,
      column: column,
      error: e,
      message: msg,
    });
  }
}

export function parse(buffer) {
  let chars = new antlr.InputStream(buffer);
  let lexer = new OttoLexer(chars);
  let tokens = new antlr.CommonTokenStream(lexer);
  let parser = new Otto(tokens);
  parser.buildParseTrees = true;
  parser.removeErrorListeners();

  const errorListener = new JestListener();
  parser.addErrorListener(errorListener);

  let tree = parser.pipeline();
  tree.accept(new Visitor());
  return errorListener.errors;
}
