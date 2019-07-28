import Runtime from '@otto-parser/Runtime'
import Step from '@otto-parser/Step'
import FileCapture from '@otto-parser/FileCapture'

export default class Stage {
  public name: string
  protected before: Stage
  protected after: Stage
  public runtime: Runtime
  protected steps: Step[] = []
  protected capture: Map<string, FileCapture>
  protected restore: String[]
}
