const fs = require('fs');
const antlr = require('antlr4');

const Lexer = require('./build/parser/JavaScript/OttoLexer').OttoLexer;
const Parser = require('./build/parser/JavaScript/Otto').Otto;
const OttoListener = require('./build/parser/JavaScript/OttoListener').OttoListener;

const input = fs.readFileSync('./examples/webapp.otto', 'utf8');
let chars = new antlr.InputStream(input);
let lexer = new Lexer(chars);
let tokens = new antlr.CommonTokenStream(lexer);
let parser = new Parser(tokens);
parser.buildParseTrees = true;
let tree = parser.pipeline();

class Visitor {
  visitChildren(ctx) {
    if (!ctx) {
      console.log('noctx');
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

class L extends OttoListener {
  enterPipeline(ctx) {
    console.log('entering pipeline');
  }

  enterConfigure_block(ctx) {
    console.log('enter config');
  }

  enterUse_block(ctx) {
    console.log('enter use');
  }
}

tree.accept(new Visitor());
//antlr.tree.ParseTreeWalker.DEFAULT.walk(new L(), tree);
