import { appConfig } from "./config";

// TS Fixture Main

function startService(): void {
  console.log(`Starting ${appConfig.serviceName}`);
  console.log(`Has key: ${appConfig.openaiKey.length > 0}`);
}

startService();
