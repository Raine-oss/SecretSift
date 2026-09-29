// Java Fixture App

public class App {
    public static void main(String[] args) {
        System.out.println("Running " + Config.getAppName());
        String token = "ghp_123456789012345678901234567890123456";
        System.out.println("Token initialized: " + (token != null));
    }
}
