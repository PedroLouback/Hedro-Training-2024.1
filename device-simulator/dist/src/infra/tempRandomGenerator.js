"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.TempRandomGenerator = void 0;
function random(min, max) {
    return Math.floor(min + Math.random() * (max - min));
}
class TempRandomGenerator {
    constructor(logger) {
        this.logger = logger;
        this.TEMP_MIN = 10;
        this.TEMP_MAX = 15;
    }
    generate() {
        this.logger.debug('generated random data');
        const randomValue = random(this.TEMP_MIN, this.TEMP_MAX);
        return randomValue;
    }
}
exports.TempRandomGenerator = TempRandomGenerator;
