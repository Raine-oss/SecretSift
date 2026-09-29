import config
from services.client import make_request

# Python Fixture Main

def main():
    print(config.get_status())
    res = make_request()
    print(f"Loaded client with token length: {len(res['token'])}")

if __name__ == "__main__":
    main()
