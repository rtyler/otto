/*
 * This test file will verify the parsing behavior of the pipeline block
 */

import fs from 'fs';
import path from 'path';

import { MIN_PIPELINE, parse } from '../utils';

describe('pipeline {}', () => {
  it('should pass with the minimum viable pipeline', () => {
    expect(parse(MIN_PIPELINE)).toHaveLength(0);
  });

  describe('with full examples', () => {
    const examples_dir = path.join(__dirname, '..', '..', 'examples');
    fs.readdirSync(examples_dir).filter(filename => filename.endsWith('.otto')).map((filename) => {
      it(`should be able to parse ${filename}`, () => {
        const buffer = fs.readFileSync(path.join(examples_dir, filename), 'utf8');
        expect(parse(buffer)).toHaveLength(0);
      });
    });
  });
});
