export class Logger {
  static log(message: string) {
    console.log(`[Rivet] ${message}`);
  }

  static warning(message: string) {
    console.warn(`[Rivet] ${message}`);
  }

  static error(message: string) {
    console.error(`[Rivet] ${message}`);
  }
}
