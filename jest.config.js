module.exports = {
  preset: 'ts-jest',
  testEnvironment: 'node',
  moduleNameMapper: {
    "^@otto/grammar/(.*)$": "<rootDir>/grammar/build/parser/JavaScript/$1",
    "^@otto/(.*)$": "<rootDir>/lib/src/$1",
    "^@otto-orchestrator/(.*)$": "<rootDir>/services/orchestrator/src/$1",
  },
};
