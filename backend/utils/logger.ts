enum LogLevel {
  INFO = 'INFO',
  WARN = 'WARN',
  ERROR = 'ERROR',
  DEBUG = 'DEBUG'
}

class Logger {
  private static formatMessage(level: LogLevel, message: string): string {
    return `[${new Date().toISOString()}] [${level}] ${message}`;
  }

  static log(level: LogLevel, message: string, metadata?: Record<string, unknown>) {
    const formattedMessage = this.formatMessage(level, message);
    
    switch (level) {
      case LogLevel.ERROR:
        console.error(formattedMessage, metadata);
        break;
      case LogLevel.WARN:
        console.warn(formattedMessage, metadata);
        break;
      case LogLevel.INFO:
        console.info(formattedMessage, metadata);
        break;
      case LogLevel.DEBUG:
        console.debug(formattedMessage, metadata);
        break;
    }
  }

  static info(message: string, metadata?: Record<string, unknown>) {
    this.log(LogLevel.INFO, message, metadata);
  }

  static warn(message: string, metadata?: Record<string, unknown>) {
    this.log(LogLevel.WARN, message, metadata);
  }

  static error(message: string, metadata?: Record<string, unknown>) {
    this.log(LogLevel.ERROR, message, metadata);
  }

  static debug(message: string, metadata?: Record<string, unknown>) {
    this.log(LogLevel.DEBUG, message, metadata);
  }
}

export { Logger, LogLevel }; 