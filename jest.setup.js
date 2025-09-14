// Mock the native module for testing
jest.mock('react-native', () => ({
  NativeModules: {
    ReactNativeRs: {
      execute: jest.fn(),
      getAppHomeDir: jest.fn(),
    },
  },
  Platform: {
    select: jest.fn((options) => options.default || options.ios),
  },
  TurboModuleRegistry: {
    getEnforcing: jest.fn(() => ({
      execute: jest.fn(),
      getAppHomeDir: jest.fn(),
    })),
  },
}));
