// JS Fixture Entry

const config = require("./config");
const { fetchUser } = require("./api/client");

function run() {
  console.log(`Starting ${config.appName}`);
  const user = fetchUser("user_123");
  console.log(`Fetched user: ${user.id}`);
}

run();
