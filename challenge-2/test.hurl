# Test 1
# Please note, that we will use a different set of values when validating your submission
POST http://localhost:3000/plan-my-trip
{
    "destination": "Banff",
    "duration": "1 week",
    "num_people": "four",
    "activities": ["hiking", "camping", "long drive", "food"]
    
}

HTTP 201
[Asserts]
jsonpath "$.itinerary" != null
[Captures]
tag: jsonpath "$.tag"

# Test 2
# Please note, that we will use a different set of values when validating your submission
GET http://localhost:3000/{{tag}}
HTTP 200
[Asserts]
jsonpath "$.itinerary" != null
