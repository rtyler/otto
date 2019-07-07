/*
 * This file contains the parser for the Otto description language
 *
 * This is to be considered the reference grammar for all .otto files
 */
parser grammar Otto;

options {
    tokenVocab=OttoLexer;
}

// Start rule to parse the .otto pipeline declaration
pipeline
    : use_block?
    configure_block?
    envs_block?
    pipeline_block
    ;

/*
 * The use {} block helps bring user defined libraries into scope for the
 * runtime of the pipeline, but does not influence parse time
 *
 * Example:
    use {
      stdlib
    }
 *
 */
use_block
    : USE BEGIN statements? END
    ;

/*
 * The configure {} block allows the user to configure libraries or other
 * pipeline-specific settings.
 *
 * Example:
    configure {
      slack {
        channel = '#otto'
      }
    }
 */
configure_block
    : CONFIGURE BEGIN setting_block* END
    ;

/* The environments {} block allows the definition of logical environments for
 * the pipeline to deliver into
 *
 * Example:
    environments {
      preprod {
        settings {
          HOSTNAME = "preprod-ottoapp.herokuapp.com"
        }
      }
    }
 */
envs_block
    : ENVIRONMENTS BEGIN env_block+ END
    ;

/*
 * Handling an identified environment block.
 *
 * This block is typically responsible for configuring a single target
 * environment for the delivery of this pipeline.
 *
 * Example:
    preprod {
      settings {
        HOSTNAME = "preprod-ottoapp.herokuapp.com"
      }
    }
 */
env_block
    : ID BEGIN settings_block? END
    ;
settings_block
    :  SETTINGS BEGIN settings? END
    ;

/*
 * Set settings for an identified subcomponent
 *
 * Example:
      slack {
        channel = '#otto'
      }
 *
 * The identified subcomponent is not expected to be known at parse time, but
 * should be looked up when the parsed .otto file has been processed to ensure
 * that a corresponding subcomponent is available
 */
setting_block
    : ID BEGIN settings? END
    ;
settings
    : setting+
    ;
setting
    : ID ASSIGN (StringLiteral | array | macro | macroKeywords)
    ;
array
    : ARRAY_START (StringLiteral COMMA?)+ ARRAY_END
    ;



/*
 * The pipeline {} block contains the main execution definition of the
 * pipeline. Roughly modeled after the Jenkins Pipeline declarative syntax.
 */
pipeline_block
    : PIPELINE BEGIN stages_block END
    ;

stages_block
    : STAGES BEGIN (macro? stages macro?)+ END
    ;
stages
    : STAGE BEGIN stageStatements* END
    ;

stageStatements
    : settings
    | steps
    | runtime
    | cache
    | gates
    | deployExpr
    | notifyExpr
    | macro+
    // And finally, allow nesting our stages!
    | stages+
    ;
steps
    : STEPS BEGIN statements+ END
    ;
cache
    : CACHE BEGIN
        (
        (setting+)
        | fromExpr
        | cacheUseExpr
        )
     END
    ;
/*
 * cache {} `use` expressions allow stages to pull in cached entries from
 * elsewhere
 */
cacheUseExpr
    : USE ID
    ;

runtime
    : RUNTIME BEGIN
        (
        setting_block
        | fromExpr
        )
    END
    ;
/*
 * XXX: This syntax requires some test coverage to ensure that the grammar
 * allows for order independence properly, while still restricting only a
 * single enter block, for example
 */
gates
    : GATES BEGIN
    (
    enter
    | exit
    | fromExpr
    )+
    END
    ;
enter
    : ENTER BEGIN enterExpr+ END
    ;
exit
    : EXIT BEGIN exitExpr+ END
    ;
enterExpr
    : (BRANCH EQUALS StringLiteral)
    | statements
    | setting_block
    ;
exitExpr
    : statements
    | setting_block
    ;

/*
 * A "deployment expression" signifies that the output of the given context
 * will result in binaries or some form of delivery to the environment being
 * pointed to
 */
deployExpr
    : ENVIRONMENT TO ID
    ;


notifyExpr
    : NOTIFY BEGIN
        (
        (SUCCESS | FAILURE | COMPLETE)
        BEGIN
        statements+
        END
        )+
    END
    ;


/*
 * A "from" expression is a shorthand in the syntax for coping the contents of
 * another block of "this" type, from another stage or location
 *
 * For exmaple, if one stage in the pipeline has a `cache` configuration
 * defined, a later stage can use: cache { from 'StageA' } to copy the settings
 * over verbatim
 */
fromExpr
    : FROM StringLiteral
    ;

statements
    : statement+
    ;
statement
    : keyword
    | step
    | StringLiteral
    ;

step
    : ID StringLiteral
    ;


/*
 * Macro expressions can be a single line, or a single line with a block
 * attached
 */
macro
    : ID OPEN macroArguments CLOSE
    (BEGIN (stages+)? END)?
    ;
macroArguments
    :
    (
        (StringLiteral | macroKeywords)
    COMMA?
    )+
    ;
macroKeywords
    : IT
    ;



/*
 * Keywords are expected to be semantically important after parse time and
 * effectively represent reserved words in the .otto language
 */
keyword
    : STDLIB
    ;
