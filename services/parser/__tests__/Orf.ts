
import { Orf } from '../src/Orf'

describe('Orf', () => {
  it('should have a version', () => {
    expect((new Orf()).version).toBeGreaterThan(0)
  })

  describe('handling libraries', () => {
    describe('.libraries', () => {
      it('should return an empty array by default', () => {
        const o = new Orf()
        expect(o.libraries).toHaveLength(0)
      })
    })
  });

  describe('handling runtimes', () => {
    describe('.runtimes', () => {
      it('should return an empty array by default', () => {
        const o = new Orf()
        expect(o.runtimes).toHaveLength(0)
      })
    });
  });

  describe('when serialized', () => {
    it('should serialize by default', () => {
      const o = new Orf()
      expect(JSON.stringify(o)).toBeTruthy()
    })
  })
})
