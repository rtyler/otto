/*
 * Otto Representation Format: this file contains the necessary to maintain the
 * internal representation of a parsed .otto file.
 *
 * See the parser README for more information
 */

enum LibraryType {
  Builtin,
  FileReference,
}

class Library {
  protected readonly libraryType: LibraryType;
  protected readonly libraryRef: string;
}


class Setting {
  protected readonly encrypted: Boolean;
  protected readonly value: any;
}


class Configuration {
  protected readonly settings: Map<string, Setting>;
}

interface Runtime {
}

interface Step {
}

interface FileCapture {
}

class Stage {
  protected readonly name: string;
  protected before: Stage;
  protected after: Stage;
  protected runtime: Runtime;
  protected steps: Array<Step> = [];
  protected capture: Map<string, FileCapture>;
  protected restore: Array<string>;
}

/**
 * Orf is the Otto Representation Format and acts as the parsed and serialized
 * format of a .otto file, suitable for consumption by components within Otto.
 */
export default class Orf {
  /** The version field is for system compatibility */
  static readonly version = 1;
  /**
   * An array of libraries which must be loaded at runtime
   */
  protected libraries: Array<Library> = [];

  /**
   * A map of configuration objects for configuring arbitrary
   * steps/libraries/etc
   */
  protected configuration: Map<string, Configuration>;

  /**
   * An ordered array of runtimes which will be used throughout the process
   */
  protected runtimes: Array<Runtime> = [];

  /**
   * An ordered array of stages as they have been parsed, not necessary how
   * they will be executed which may be more of a directed graph.
   */
  protected stages: Array<Stage> = [];
}
