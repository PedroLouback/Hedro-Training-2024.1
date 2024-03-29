"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.LoggerInitializer = void 0;
const pino_1 = __importDefault(require("pino"));
class LoggerInitializer {
    init() {
        return (0, pino_1.default)({
            // eslint-disable-next-line @typescript-eslint/strict-boolean-expressions
            level: process.env.LOG_LEVEL || 'debug'
        });
    }
}
exports.LoggerInitializer = LoggerInitializer;
