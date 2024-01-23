"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.TempRandomGenerator = void 0;
class TempRandomGenerator {
    constructor(logger) {
        this.logger = logger;
        this.TEMP_MIN = 10;
        this.TEMP_MAX = 15;
    }
    generate() {
        this.logger.debug('generated random data');
        return 0;
    }
}
exports.TempRandomGenerator = TempRandomGenerator;
