/**
 * Centralized logging utility for the application
 *
 * Benefits:
 * - Easy to disable logs in production
 * - Categorized logging (error, warn, info, debug)
 * - Can be extended to send logs to external services
 * - Type-safe logging with TypeScript
 */

type LogLevel = 'error' | 'warn' | 'info' | 'debug';

interface LogConfig {
  enabled: boolean;
  levels: Set<LogLevel>;
  includeTimestamp: boolean;
}

class Logger {
  private config: LogConfig = {
    enabled: import.meta.env.DEV, // Only enable in development by default
    levels: new Set(['error', 'warn', 'info', 'debug']),
    includeTimestamp: true,
  };

  private formatMessage(level: LogLevel, message: string, ..._args: any[]): string {
    const timestamp = this.config.includeTimestamp
      ? `[${new Date().toISOString()}]`
      : '';
    const levelTag = `[${level.toUpperCase()}]`;
    return `${timestamp}${levelTag} ${message}`;
  }

  private shouldLog(level: LogLevel): boolean {
    return this.config.enabled && this.config.levels.has(level);
  }

  /**
   * Log an error message
   * Always shown, even in production
   */
  error(message: string, ...args: any[]): void {
    if (this.shouldLog('error') || !import.meta.env.DEV) {
      console.error(this.formatMessage('error', message), ...args);
    }
  }

  /**
   * Log a warning message
   * Shown in development and production
   */
  warn(message: string, ...args: any[]): void {
    if (this.shouldLog('warn') || !import.meta.env.DEV) {
      console.warn(this.formatMessage('warn', message), ...args);
    }
  }

  /**
   * Log an info message
   * Only shown in development
   */
  info(message: string, ...args: any[]): void {
    if (this.shouldLog('info')) {
      console.info(this.formatMessage('info', message), ...args);
    }
  }

  /**
   * Log a debug message
   * Only shown in development
   */
  debug(message: string, ...args: any[]): void {
    if (this.shouldLog('debug')) {
      console.debug(this.formatMessage('debug', message), ...args);
    }
  }

  /**
   * Configure the logger
   */
  configure(config: Partial<LogConfig>): void {
    this.config = { ...this.config, ...config };
  }

  /**
   * Disable all logging
   */
  disable(): void {
    this.config.enabled = false;
  }

  /**
   * Enable logging
   */
  enable(): void {
    this.config.enabled = true;
  }
}

// Export singleton instance
export const logger = new Logger();

// Export convenience functions
export const logError = (message: string, ...args: any[]) => logger.error(message, ...args);
export const logWarn = (message: string, ...args: any[]) => logger.warn(message, ...args);
export const logInfo = (message: string, ...args: any[]) => logger.info(message, ...args);
export const logDebug = (message: string, ...args: any[]) => logger.debug(message, ...args);
