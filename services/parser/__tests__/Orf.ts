
import Orf from '../src/Orf'

describe('Orf', () => {
  it('should instantiate', () => {
    expect(new Orf()).toBeTruthy()
  })

  it('should have a version', () => {
    expect((new Orf()).version).toBeGreaterThan(0)
  })

  describe('when serialized', () => {
    it('should serialize by default', () => {
      const o = new Orf()
      expect(JSON.stringify(o)).toBeTruthy()
    })
  })
})
