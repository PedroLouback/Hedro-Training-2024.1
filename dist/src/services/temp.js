"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.TempGeneratorService = void 0;
class TempGeneratorService {
    constructor(randomGenerator, messaging) {
        this.randomGenerator = randomGenerator;
        this.messaging = messaging;
        const tempDataInterval = process.env.TEMP_DATA_INTERVAL;
        if (tempDataInterval === undefined || tempDataInterval === null) {
            throw new Error('invalid temperature data interval');
        }
        this.TEMP_DATA_INTERVAL = Number(tempDataInterval);
    }
    do() {
        setInterval(() => {
            const random = this.randomGenerator.generate();
            const data = {
                device: 'random',
                type: 'TEMP',
                value: String(random)
            };
            this.messaging.pub(data);
        }, this.TEMP_DATA_INTERVAL);
    }
}
exports.TempGeneratorService = TempGeneratorService;
