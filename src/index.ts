import { NativeModules, Platform } from 'react-native';

const LINKING_ERROR =
  'The package \'react-native-rs\' doesn\'t seem to be linked. Make sure: \n\n' +
  Platform.select({ ios: '- You have run \'pod install\'\n', default: '' }) +
  '- You rebuilt the app after installing the package\n' +
  '- You are not using Expo Go\n';

declare global {
  interface Global {
    // eslint-disable-next-line @typescript-eslint/naming-convention
    __turboModuleProxy?: any;
  }
}

// Check if TurboModules are enabled
const isTurboModuleEnabled = (global as any).__turboModuleProxy != null;

const ReactNativeRsModule = isTurboModuleEnabled
  ? require('./NativeReactNativeRs').default
  : NativeModules.ReactNativeRs;

const ReactNativeRs = ReactNativeRsModule || new Proxy(
  {},
  {
    get() {
      throw new Error(LINKING_ERROR);
    }
  }
);

/**
 * Command interface for Rust bridge operations
 */
export interface Command {
  cmd: string;
  params?: any;
}

/**
 * Result interface for Rust bridge operations
 */
export interface CommandResult<T = any> {
  res?: T;
  error?: string;
  panic: boolean;
  panic_details?: {
    cmd: string;
    msg: string;
  };
}

/**
 * Main RustBridge class for executing commands
 */
export class RustBridge {
  /**
   * Execute a command in Rust
   * @param command - The command object with cmd and optional params
   * @returns Promise resolving to the command result
   */
  static async execute<T = any>(command: Command): Promise<T> {
    const cmdString = JSON.stringify(command);
    const resultString = await ReactNativeRs.execute(cmdString);
    const result: CommandResult<T> = JSON.parse(resultString);
    
    if (result.panic) {
      throw new Error(
        `Rust panic in command '${result.panic_details?.cmd}': ${result.panic_details?.msg}`
      );
    }
    
    if (result.error) {
      throw new Error(`Rust error: ${result.error}`);
    }
    
    return result.res as T;
  }

  /**
   * Get the app's home directory
   * @returns Promise resolving to the home directory path
   */
  static async getAppHomeDir(): Promise<string> {
    return ReactNativeRs.getAppHomeDir();
  }
}

// Export types and main class
export { Command, CommandResult };
export default RustBridge;
