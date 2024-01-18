"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.HumidityRandomGenerator = void 0;
function random(min, max) {
    return (min + Math.random() * (max - min));
}
class HumidityRandomGenerator {
    constructor(logger) {
        this.logger = logger;
        this.HUMIDITY_MIN = 0.3;
        this.HUMIDITY_MAX = 0.8;
    }
    generate() {
        this.logger.debug('generated random data');
        const randomValue = random(this.HUMIDITY_MIN, this.HUMIDITY_MAX);
        return Number(randomValue.toFixed(1));
    }
}
exports.HumidityRandomGenerator = HumidityRandomGenerator;
