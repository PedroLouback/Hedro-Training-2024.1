"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Messaging = void 0;
const mqtt_1 = require("mqtt");
class Messaging {
    constructor(logger) {
        this.logger = logger;
        const mqttHost = process.env.MQTT_HOST;
        const mqttProtocol = process.env.MQTT_PROTOCOL;
        // eslint-disable-next-line @typescript-eslint/prefer-optional-chain
        if (mqttHost === undefined || mqttHost === null || mqttProtocol === undefined || mqttProtocol === null) {
            throw new Error('invalid mqtt credetials');
        }
        this.MQTT_HOST = mqttHost;
        this.MQTT_PROTOCOL = mqttProtocol;
    }
    pub(data) {
        this.logger.debug(`publishing: ${JSON.stringify(data)}`);
        this.conn.publish(`HedroTraining2024/${data.device}/${data.type}`, JSON.stringify(data));
        this.logger.debug('published!');
    }
    connect() {
        try {
            this.conn = (0, mqtt_1.connect)(`${this.MQTT_PROTOCOL}://${this.MQTT_HOST}`);
        }
        catch (err) {
            this.logger.error({ msg: 'something went wrong', error: err });
            throw err;
        }
    }
}
exports.Messaging = Messaging;
