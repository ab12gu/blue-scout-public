import json
import requests

response = requests.get("https://www.thebluealliance.com/swagger/api_v3.json", stream=True)

with open("api_v3.json", "wb") as file:
    for chunk in response.iter_content(chunk_size=8192):
        file.write(chunk)

def remove_if_none_match(openapi_spec):
    # Remove from paths
    for path, methods in openapi_spec.get("paths", {}).items():
        for method, details in methods.items():
            if "parameters" in details:
                details["parameters"] = [
                    param for param in details["parameters"]
                    if param.get("name") != "If-None-Match" and param.get("$ref") != "#/components/parameters/If-None-Match"
                ]

    # Remove from components
    if "components" in openapi_spec and "parameters" in openapi_spec["components"]:
        openapi_spec["components"]["parameters"] = {
            key: value for key, value in openapi_spec["components"]["parameters"].items()
            if value.get("name") != "If-None-Match"
        }

def load_openapi_spec(file_path):
    with open(file_path, "r", encoding="utf-8") as f:
        return json.load(f)

def save_openapi_spec(file_path, openapi_spec):
    with open(file_path, "w", encoding="utf-8") as f:
        json.dump(openapi_spec, f, indent=2)

openapi_spec = load_openapi_spec("api_v3.json")
remove_if_none_match(openapi_spec)
save_openapi_spec("api_v3.json", openapi_spec)
