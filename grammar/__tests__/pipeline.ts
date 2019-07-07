/*
 * This test file will verify the parsing behavior of the pipeline block
 */

import { MIN_PIPELINE, parse } from '../utils';

describe('pipeline {}', () => {
  it('should pass with the minimum viable pipeline', () => {
    expect(parse(MIN_PIPELINE)).toHaveLength(0);
  });
});
