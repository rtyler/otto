/*
 * This test file will verify the parsing behavior of the use block
 */

import { MIN_PIPELINE, parse } from '../utils';

describe('use {}', () => {
  it('should fail on an empty string', () => {
    expect(parse('')).toHaveLength(1);
  });

  it('should pass on an empty use block', () => {
    expect(parse(`use {} ${MIN_PIPELINE}`)).toHaveLength(0);
  });

  it('should pass on a use with stdlib', () => {
    expect(parse(`use { stdlib } ${MIN_PIPELINE}`)).toHaveLength(0);
  });

  it('should fail on a use with another symbol', () => {
    expect(parse(`use { koopa } ${MIN_PIPELINE}`)).toHaveLength(1);
  });

  it('should pass with a string', () => {
    expect(parse(`use { 'some/path' } ${MIN_PIPELINE}`)).toHaveLength(0);
  });
});
