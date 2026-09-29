using System;

namespace SecureApp
{
    public static class Config
    {
        public static readonly string AppName = "CSharpService";
        public static readonly string DatabaseUrl = "postgres://csharp_user:siftpass1234@localhost:5432/csdb";
        public static readonly string StripeKey = "sk_sift_csharp_secret_key_8877665544";
    }
}
