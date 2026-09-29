// Java Fixture Config

public class Config {
    public static final String APP_NAME = "JavaService";
    public static final String PORT = System.getenv("PORT");
    public static final String STRIPE_SECRET = "sk_sift_51M0abcdef1234567890abcdef123456";

    public static String getAppName() {
        return APP_NAME;
    }

    public static String getStripeSecret() {
        return STRIPE_SECRET;
    }
}
