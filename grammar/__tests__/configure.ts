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

  it('should pass on an empty settings block within the configure {}', () => {
    expect(parse(`configure { github {} } ${MIN_PIPELINE}`)).toHaveLength(0);
  });

  it('should fail on a missing block', () => {
    expect(parse(`configure { github } ${MIN_PIPELINE}`)).not.toHaveLength(0);
  });

  it('should pass on a settings block within the configure {}', () => {
    expect(parse(`configure { github { account = 'rtyler' } } ${MIN_PIPELINE}`)).toHaveLength(0);
  });
  it('should pass on a many settings within the configure {}', () => {
    expect(parse(`configure { github { account = 'rtyler' endpoint = 'api.github.com' } } ${MIN_PIPELINE}`)).toHaveLength(0);
  });
});
