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
    : CONFIGURE BEGIN setting_block+ END
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
    : ID ASSIGN (StringLiteral | array)
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
    : STAGES BEGIN stages+ END
    ;
stages
    : STAGE OPEN StringLiteral CLOSE BEGIN stageStatements* END
    ;


stageStatements
    : steps
    | runtime
    | cache
    | when
    | deployExpr
    | notify
    | feedback
    | before
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
        )
     END
    ;
runtime
    : RUNTIME BEGIN 
        (
        setting_block
        | fromExpr
        )
    END
    ;
when
    : WHEN BEGIN whenExpr* END
    ;
whenExpr
    : (BRANCH EQUALS StringLiteral)
    | fromExpr
    ;

/*
 * A "deployment expression" signifies that the output of the given context
 * will result in binaries or some form of delivery to the environment being
 * pointed to
 */
deployExpr
    : ENVIRONMENT TO ID
    ;


notify
    : NOTIFY BEGIN
        (
        (SUCCESS | FAILURE | COMPLETE)
        BEGIN
        statements+
        END
        )+
    END
    ;

feedback
    : FEEDBACK BEGIN
        (
        statements
        | setting_block
        )+
    END
    ;

before
    : BEFORE BEGIN statements+ END
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
 * Keywords are expected to be semantically important after parse time and
 * effectively represent reserved words in the .otto language
 */
keyword
    : STDLIB
    ;
