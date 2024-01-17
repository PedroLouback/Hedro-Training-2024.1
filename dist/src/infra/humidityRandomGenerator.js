"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.HumidityRandomGenerator = void 0;
class HumidityRandomGenerator {
    constructor(logger) {
        this.logger = logger;
        this.HUMIDITY_MIN = 0.3;
        this.HUMIDITY_MAX = 0.8;
    }
    generate() {
        this.logger.debug('generated random data');
        return 0;
    }
}
exports.HumidityRandomGenerator = HumidityRandomGenerator;
