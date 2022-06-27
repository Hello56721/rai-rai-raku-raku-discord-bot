"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const discord_js_1 = __importDefault(require("discord.js"));
const secrets_json_1 = __importDefault(require("../data/secrets.json"));
const annoying_1 = __importDefault(require("./commands/annoying"));
let commands = new Map();
function logMessage(message) {
    let messageAuthor = message.author.tag;
    let messageChannel = "";
    if (message.channel.isText()) {
        if (message.guild != null) {
            messageChannel = message.channel.name;
        }
        else {
            messageChannel = message.channel.recipient.tag;
        }
    }
    let messageGuild = "";
    if (message.guild != null) {
        messageGuild = message.guild.nameAcronym;
    }
    console.log(`[MESSAGE ${messageAuthor} ${messageGuild} ${messageChannel}]: ${message.content}`);
}
annoying_1.default.registerCommand(commands);
let client = new discord_js_1.default.Client({
    intents: [
        discord_js_1.default.Intents.FLAGS.GUILDS,
        discord_js_1.default.Intents.FLAGS.GUILD_MESSAGES,
        discord_js_1.default.Intents.FLAGS.DIRECT_MESSAGES
    ]
});
client.once("ready", (client) => {
    console.log(`[INFO]: Logged in to Discord as ${client.user.tag}.`);
});
client.on("messageCreate", (message) => {
    logMessage(message);
    let commandName = message.content.split(" ")[0];
    let commandArguments = message.content.substring(commandName.length + 1);
    let commandHandler = commands.get(commandName);
    if (commandHandler != undefined) {
        commandHandler(message, commandArguments);
    }
    else {
        // Do other stuff that are not command related.
    }
});
client.login(secrets_json_1.default.TOKEN);
//# sourceMappingURL=main.js.map