/*
 *
 */
export default class Runtime {
  public readonly runtimeType: string
  public readonly settings: Object


  constructor(t: string, settings: Object) {
    this.runtimeType = t
    this.settings = settings
  }
}