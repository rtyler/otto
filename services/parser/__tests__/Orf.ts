
import Orf from '../src/Orf';

describe('Orf', () => {
  it('should instantiate', () => {
    expect(new Orf()).toBeTruthy();
  });

  it('should have a version', () => {
    expect(Orf.version).toBeGreaterThan(0);
  });
});
