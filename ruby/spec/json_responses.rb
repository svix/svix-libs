# rubyfmt: false
ListResponseAppOut_JSON = '{
  "data": [
    {
      "uid": "unique-identifier",
      "name": "My first application",
      "rateLimit": 0,
      "id": "app_1srOrx2ZWZBpBUvZwXKQmoEYga2",
      "createdAt": "2019-08-24T14:15:22Z",
      "updatedAt": "2019-08-24T14:15:22Z",
      "metadata": {
        "property1": "string",
        "property2": "string"
      }
    }
  ],
  "iterator": "iterator",
  "prevIterator": "-iterator",
  "done": true
}'
ListResponseMessageAttemptOut_JSON = '{
  "data": [
    {
      "url": "https://example.com/webhook/",
      "response": "{}",
      "responseStatusCode": 200,
      "responseDurationMs": 0,
      "status": 0,
      "triggerType": 0,
      "msgId": "msg_1srOrx2ZWZBpBUvZwXKQmoEYga2",
      "endpointId": "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2",
      "id": "atmpt_1srOrx2ZWZBpBUvZwXKQmoEYga2",
      "timestamp": "2019-08-24T14:15:22Z",
      "msg": {
        "eventId": "unique-identifier",
        "eventType": "user.signup",
        "payload": {
          "email": "test@example.com",
          "type": "user.created",
          "username": "test_user"
        },
        "channels": [
          "project_123",
          "group_2"
        ],
        "id": "msg_1srOrx2ZWZBpBUvZwXKQmoEYga2",
        "timestamp": "2019-08-24T14:15:22Z",
        "tags": [
          "project_1337"
        ]
      }
    }
  ],
  "iterator": "iterator",
  "prevIterator": "-iterator",
  "done": true
}'
ReplayOut_JSON = '
{
  "id": "qtask_1srOrx2ZWZBpBUvZwXKQmoEYga2",
  "status": "running",
  "task": "endpoint.replay"
}'
ListResponseMessageOut_JSON = '{
  "data": [
    {
      "eventId": "unique-identifier",
      "eventType": "user.signup",
      "payload": {
        "email": "test@example.com",
        "type": "user.created",
        "username": "test_user"
      },
      "channels": [
        "project_123",
        "group_2"
      ],
      "id": "msg_1srOrx2ZWZBpBUvZwXKQmoEYga2",
      "timestamp": "2019-08-24T14:15:22Z",
      "tags": [
        "project_1337"
      ]
    }
  ],
  "iterator": "iterator",
  "prevIterator": "-iterator",
  "done": true
}'
MessageOut_JSON='{
  "eventId": "unique-identifier",
  "eventType": "user.signup",
  "payload": {
    "email": "test@example.com",
    "type": "user.created",
    "username": "test_user"
  },
  "channels": [
    "project_123",
    "group_2"
  ],
  "id": "msg_1srOrx2ZWZBpBUvZwXKQmoEYga2",
  "timestamp": "2019-08-24T14:15:22Z",
  "tags": [
    "project_1337"
  ]
}'
ListResponseOperationalWebhookEndpointOut_JSON='{
  "data": [
    {
      "id": "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2",
      "description": "string",
      "rateLimit": 0,
      "uid": "unique-identifier",
      "url": "https://example.com/webhook/",
      "disabled": false,
      "filterTypes": [
        "message.attempt.failing"
      ],
      "createdAt": "2019-08-24T14:15:22Z",
      "updatedAt": "2019-08-24T14:15:22Z",
      "metadata": {
        "property1": "string",
        "property2": "string"
      }
    }
  ],
  "iterator": "iterator",
  "prevIterator": "-iterator",
  "done": true
}'
ListResponseBackgroundTaskOut_JSON='{
  "data": [
    {
      "data": {},
      "id": "qtask_1srOrx2ZWZBpBUvZwXKQmoEYga2",
      "status": "running",
      "task": "endpoint.replay"
    }
  ],
  "iterator": "iterator",
  "prevIterator": "-iterator",
  "done": true
}'
EventTypeImportOpenApiOut_JSON='{
  "data": {
    "modified": [
      "user.signup"
    ],
    "to_modify": [
      {
        "name": "user.signup",
        "description": "string",
        "schemas": {
          "description": "An invoice was paid by a user",
          "properties": {
            "invoiceId": {
              "description": "The invoice id",
              "type": "string"
            },
            "userId": {
              "description": "The user id",
              "type": "string"
            }
          },
          "required": [
            "invoiceId",
            "userId"
          ],
          "title": "Invoice Paid Event",
          "type": "object"
        },
        "deprecated": true,
        "featureFlag": "cool-new-feature",
        "groupName": "user"
      }
    ]
  }
}'
EndpointOut_JSON='{
  "id": "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2",
  "metadata": {
    "property1": "string",
    "property2": "string"
  },
  "description": "string",
  "rateLimit": 0,
  "uid": "unique-identifier",
  "url": "https://example.com/webhook/",
  "version": 1,
  "disabled": false,
  "filterTypes": [
    "user.signup",
    "user.deleted"
  ],
  "channels": [
    "project_123",
    "group_2"
  ],
  "createdAt": "2019-08-24T14:15:22Z",
  "updatedAt": "2019-08-24T14:15:22Z"
}'
ListResponseMessageAttemptOut_without_msg_JSON='{
      "data": [
        {
          "url": "https://example.com/webhook/",
          "response": "{}",
          "responseStatusCode": 200,
          "responseDurationMs": 0,
          "status": 0,
          "triggerType": 0,
          "msgId": "msg_1srOrx2ZWZBpBUvZwXKQmoEYga2",
          "endpointId": "ep_1srOrx2ZWZBpBUvZwXKQmoEYga2",
          "id": "atmpt_1srOrx2ZWZBpBUvZwXKQmoEYga2",
          "timestamp": "2019-08-24T14:15:22Z"
        }
      ],
      "iterator": "iterator",
      "prevIterator": "-iterator",
      "done": true
}'