package svix

import (
	"github.com/svix/svix-webhooks/go/models"
)

// Instantiates a new MessageIn object with a raw string payload.
// The payload is not normalized on the server. Normally, payloads are required
// to be JSON, and Svix will minify the payload before sending the webhook
// (for example, by removing extraneous whitespace or unnecessarily escaped
// characters in strings). With this function, the payload will be sent
// "as is", without any minification or other processing.
//
// The `contentType` parameter can be used to change the `content-type` header
// of the webhook sent by Svix overriding the default of `application/json`.
//
// See the class documentation for details about the other parameters.
func NewMessageInRaw(
	eventType string,
	payload string,
	contentType *string,
) *models.MessageIn {
	msgIn := models.MessageIn{}

	transformationsParams := map[string]interface{}{
		"rawPayload": payload,
	}
	if contentType != nil {
		transformationsParams["headers"] = map[string]string{
			"content-type": *contentType,
		}
	}
	msgIn.TransformationsParams = &transformationsParams

	return &msgIn
}
