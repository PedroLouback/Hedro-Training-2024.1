"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.HumidityGeneratorService = void 0;
class HumidityGeneratorService {
    constructor(randomGenerator, messaging) {
        this.randomGenerator = randomGenerator;
        this.messaging = messaging;
        const dataInterval = process.env.HUMIDITY_DATA_INTERVAL;
        if (dataInterval === undefined || dataInterval === null) {
            throw new Error('invalid humidity data interval');
        }
        this.HUMIDITY_DATA_INTERVAL = Number(dataInterval);
    }
    do() {
        setInterval(() => {
            const random = this.randomGenerator.generate();
            const data = {
                device: 'random',
                type: 'HUMIDITY',
                value: String(random)
            };
            this.messaging.pub(data);
        }, this.HUMIDITY_DATA_INTERVAL);
    }
}
exports.HumidityGeneratorService = HumidityGeneratorService;
