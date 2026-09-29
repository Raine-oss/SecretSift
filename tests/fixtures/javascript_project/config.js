// JS Fixture Config

const config = {
  appName: "NodeExpressService",
  port: process.env.PORT || 3000,
  stripeKey: "sk_sift_51ABCDEF1234567890abcdef123456",
  databaseUrl: "postgres://postgres:secretpassword123@db.prod:5432/main"
};

module.exports = config;
