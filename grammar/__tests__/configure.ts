/*
 * This test file will verify the parsing behavior of the configure block
 */

import { MIN_PIPELINE, parse } from '../utils';

describe('configure {}', () => {
  it('should pass without a configure block', () => {
    expect(parse(MIN_PIPELINE)).toHaveLength(0);
  });

  it('should pass with an empty configure block', () => {
    expect(parse(`configure {} ${MIN_PIPELINE}`)).toHaveLength(0);
  });
});
