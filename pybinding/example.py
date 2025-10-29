import json
import toon

# Simple example
data = {
    "user": {
        "id": 123,
        "name": "Ada",
        "tags": ["reading", "gaming"],
        "active": True
    }
}

json_str = json.dumps(data)
result = toon.encode(json_str)
print("=== Simple encoding ===")
print(result)

# Array of objects
data2 = {
    "items": [
        {"sku": "A1", "qty": 2, "price": 9.99},
        {"sku": "B2", "qty": 1, "price": 14.5}
    ]
}

json_str2 = json.dumps(data2)
result2 = toon.encode(json_str2)
print("\n=== Tabular format ===")
print(result2)
