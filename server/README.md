## Available routes:
### Sample Path and Response
POST /table/1/items -> create order for table. Body example: {"item_ids": [1, 2, 3]}
```
[
    {
        "id": 1,
        "item_id": 1,
        "table_number": 1,
        "preparation_time": 13,
        "status": "pending"
    },
    {
        "id": 2,
        "item_id": 2,
        "table_number": 1,
        "preparation_time": 5,
        "status": "pending"
    },
    {
        "id": 3,
        "item_id": 3,
        "table_number": 1,
        "preparation_time": 10,
        "status": "pending"
    }
]
```

PUT /table/1/items/3 -> update status of a specified item for a specified table number.
```
{
    "id": 3,
    "item_id": 3,
    "table_number": 1,
    "preparation_time": 15,
    "status": "served"
}
```

GET /table/1/items?remaining=true -> show all remaining items for a specified table number.
```
[
    {
        "id": 1,
        "item_id": 1,
        "table_number": 1,
        "preparation_time": 12,
        "status": "pending"
    },
    {
        "id": 2,
        "item_id": 2,
        "table_number": 1,
        "preparation_time": 12,
        "status": "pending"
    }
]
```

GET /table/1/items?remaining=false -> show all items for a specified table number.
```
[
    {
        "id": 1,
        "item_id": 1,
        "table_number": 1,
        "preparation_time": 12,
        "status": "pending"
    },
    {
        "id": 2,
        "item_id": 2,
        "table_number": 1,
        "preparation_time": 12,
        "status": "pending"
    },
    {
        "id": 3,
        "item_id": 3,
        "table_number": 1,
        "preparation_time": 15,
        "status": "served"
    }
]
```

GET /table/1/items/2 -> show a specified item for a specified table number.
```
{
    "id": 2,
    "item_id": 2,
    "table_number": 1,
    "preparation_time": 12,
    "status": "pending"
}
```

DELETE /table/1/items/2 -> remove a specified item for a specified table number.
```
"Item Deleted"
```