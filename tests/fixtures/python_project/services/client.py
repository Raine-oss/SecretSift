import os

# Python Fixture Client

def make_request():
    existing_token = os.getenv("EXISTING_TOKEN", "default_val")
    bot_token = "xoxp-1111111111-22222222222-3333333333333-mocktokenabcdef12345"
    return {"token": bot_token, "existing": existing_token}
