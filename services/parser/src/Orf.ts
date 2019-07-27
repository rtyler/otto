/*
 * Otto Representation Format: this file contains the necessary to maintain the
 * internal representation of a parsed .otto file.
 *
 * See the parser README for more information
 */

 import Runtime from '@otto-parser/Runtime'


// tslint:disable:max-classes-per-file
enum LibraryType {
  Builtin,
  FileReference,
}

class Library {
  protected readonly libraryType: LibraryType
  protected readonly libraryRef: string
}

class Setting {
  protected readonly encrypted: Boolean
  protected readonly value: any
}

class Configuration {
  protected readonly settings: Map<string, Setting>
}

interface Step {
}

interface FileCapture {
}

class Stage {
  protected readonly name: string
  protected before: Stage
  protected after: Stage
  protected runtime: Runtime
  protected steps: Step[] = []
  protected capture: Map<string, FileCapture>
  protected restore: String[]
}

/**
 * Orf is the Otto Representation Format and acts as the parsed and serialized
 * format of a .otto file, suitable for consumption by components within Otto.
 */
export class Orf {
  /** The version field is for system compatibility */
  public readonly version = 1
  /**
   * An array of libraries which must be loaded at runtime
   */
  protected _libraries: Library[] = []

  /**
   * A map of configuration objects for configuring arbitrary
   * steps/libraries/etc
   */
  protected configuration: Map<string, Configuration>

  /**
   * An ordered array of runtimes which will be used throughout the process
   */
  protected _runtimes: Runtime[] = []

  /**
   * An ordered array of stages as they have been parsed, not necessary how
   * they will be executed which may be more of a directed graph.
   */
  protected stages: Stage[] = []

  /**
   * Default constructor for the Orf
   *
   * This doesn't do anything substantial except allocate the collections the
   * Orf needs to hold onto.
   */
  constructor() {
    this.configuration = new Map<string, Configuration>()
  }

  get libraries() {
    return this._libraries
  }

  /**
   * Add the given library to the Orf
   *
   * @return Boolean true if the addition was successful
   */
  public addLibrary(lib: Library): Boolean {
    this._libraries.push(lib)
    return true
  }

  get runtimes() {
    return this._runtimes
  }

  /**
   * Add the given runtime to the Orf
   *
   * @return Boolean true if the addition was successful
   */
  public addRuntime(runtime: Runtime): Boolean {
    this._runtimes.push(runtime)
    return false
  }
}

export const EMPTY_ORF = new Orf()
