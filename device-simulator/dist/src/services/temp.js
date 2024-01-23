"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.TempGeneratorService = void 0;
class TempGeneratorService {
    constructor(randomGenerator, messaging) {
        this.randomGenerator = randomGenerator;
        this.messaging = messaging;
    }
    do() {
        const random = this.randomGenerator.generate();
        const data = {
            device: 'random',
            type: 'TEMP',
            value: String(random)
        };
        this.messaging.pub(data);
    }
}
exports.TempGeneratorService = TempGeneratorService;
