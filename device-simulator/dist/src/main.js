"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const humidityRandomGenerator_1 = require("./infra/humidityRandomGenerator");
const logger_1 = require("./infra/logger");
const messaging_1 = require("./infra/messaging");
const tempRandomGenerator_1 = require("./infra/tempRandomGenerator");
const humidity_1 = require("./services/humidity");
const temp_1 = require("./services/temp");
const dotenv_1 = __importDefault(require("dotenv"));
function main() {
    const { logger, tempRandom, humidityRandom, messaging } = setup();
    startup(tempRandom, humidityRandom, messaging);
    logger.info('application running');
}
function setup() {
    dotenv_1.default.config();
    const logger = new logger_1.LoggerInitializer().init();
    logger.info('starting application..');
    const messaging = new messaging_1.Messaging(logger);
    messaging.connect();
    const tempRandom = new tempRandomGenerator_1.TempRandomGenerator(logger);
    const humidityRandom = new humidityRandomGenerator_1.HumidityRandomGenerator(logger);
    return {
        logger,
        messaging,
        tempRandom,
        humidityRandom
    };
}
function startup(tempRandom, humidityRandom, messaging) {
    const tempGeneratorService = new temp_1.TempGeneratorService(tempRandom, messaging);
    const humidityRandomService = new humidity_1.HumidityGeneratorService(humidityRandom, messaging);
    tempGeneratorService.do();
    humidityRandomService.do();
}
main();
