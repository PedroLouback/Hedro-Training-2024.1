"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.HumidityGeneratorService = void 0;
class HumidityGeneratorService {
    constructor(randomGenerator, messaging) {
        this.randomGenerator = randomGenerator;
        this.messaging = messaging;
    }
    do() {
        const random = this.randomGenerator.generate();
        const data = {
            device: 'random',
            type: 'HUMIDITY',
            value: String(random)
        };
        this.messaging.pub(data);
    }
}
exports.HumidityGeneratorService = HumidityGeneratorService;
