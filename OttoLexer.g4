lexer grammar OttoLexer;

USE : 'use';
CONFIGURE : 'configure';
ENVIRONMENTS : 'environments';
ENVIRONMENT : 'environment';
SETTINGS : 'settings';
PIPELINE : 'pipeline';
STAGES : 'stages';
STAGE : 'stage';
STEPS : 'steps';
CACHE : 'cache';
RUNTIME : 'runtime';

NOTIFY : 'notify';
SUCCESS : 'success';
FAILURE : 'failure';
COMPLETE : 'complete';


FEEDBACK : 'feedback';
BEFORE : 'before';

WHEN : 'when';
BRANCH : 'branch';
EQUALS : '==';

/*
 * The "to" token helps signify the output of the current context going "to" a
 * designated environment
 */
TO : '->';

FROM : 'from';

// Keyword tokens
STDLIB: 'stdlib';

// Begin block
BEGIN : '{';
// End block
END : '}';
OPEN : '(';
CLOSE : ')';
ARRAY_START : '[';
ARRAY_END : ']';
COMMA : ',';
ASSIGN : '=';


StringLiteral: ('"' DoubleStringCharacter* '"'
             |  '\'' SingleStringCharacter* '\'')
;

fragment DoubleStringCharacter
    : ~["\\\r\n]
    | '\\' EscapeSequence
    | LineContinuation
    ;
fragment SingleStringCharacter
    : ~['\\\r\n]
    | '\\' EscapeSequence
    | LineContinuation
    ;
fragment EscapeSequence
    : CharacterEscapeSequence
    | '0' // no digit ahead! TODO
    | HexEscapeSequence
    | UnicodeEscapeSequence
    | ExtendedUnicodeEscapeSequence
    ;

fragment CharacterEscapeSequence
    : SingleEscapeCharacter
    | NonEscapeCharacter
    ;
fragment HexEscapeSequence
    : 'x' HexDigit HexDigit
    ;
fragment UnicodeEscapeSequence
    : 'u' HexDigit HexDigit HexDigit HexDigit
    ;
fragment ExtendedUnicodeEscapeSequence
    : 'u' '{' HexDigit+ '}'
    ;

fragment HexDigit
    : [0-9a-fA-F]
    ;
fragment SingleEscapeCharacter
    : ['"\\bfnrtv]
;
fragment NonEscapeCharacter
    : ~['"\\bfnrtv0-9xu\r\n]
    ;
fragment EscapeCharacter
    : SingleEscapeCharacter
    | [0-9]
    | [xu]
    ;

fragment LineContinuation
    : '\\' [\r\n\u2028\u2029]
    ;



ID : [a-zA-Z_]+ ;

// skip spaces, tabs, newlines
WS : [ \t\r\n]+ -> skip ;
MultiLineComment:               '/*' .*? '*/'             -> channel(HIDDEN);
SingleLineComment: '//' ~[\r\n\u2028\u2029]* -> channel(HIDDEN);
