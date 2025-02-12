// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"
	"time"

	"github.com/svix/svix-webhooks/go/models"
)

type Message struct {
	client *SvixHttpClient
}

type MessageListOptions struct {
	// Limit the number of returned items
	Limit *uint64
	// The iterator returned from a prior invocation
	Iterator *string
	// Filter response based on the channel.
	Channel *string
	// Only include items created before a certain date.
	Before *time.Time
	// Only include items created after a certain date.
	After *time.Time
	// When `true` message payloads are included in the response.
	WithContent *bool
	// Filter messages matching the provided tag.
	Tag *string
	// Filter response based on the event type
	EventTypes *[]string
}

type MessageCreateOptions struct {
	// When `true`, message payloads are included in the response.
	WithContent    *bool
	IdempotencyKey *string
}

type MessageGetOptions struct {
	// When `true` message payloads are included in the response.
	WithContent *bool
}

// List all of the application's messages.
//
// The `before` and `after` parameters let you filter all items created before or after a certain date. These can be used alongside an iterator to paginate over results
// within a certain window.
//
// Note that by default this endpoint is limited to retrieving 90 days' worth of data
// relative to now or, if an iterator is provided, 90 days before/after the time indicated
// by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
// set the `before` or `after` parameter as appropriate.
func (message *Message) List(
	ctx context.Context,
	appId string,
	o *MessageListOptions,
) (*models.ListResponseMessageOut, error) {
	pathMap := map[string]string{
		"app_id": appId,
	}
	queryMap := map[string]string{}
	var err error
	if o != nil {
		serializeParamToMap("limit", o.Limit, queryMap, &err)
		serializeParamToMap("iterator", o.Iterator, queryMap, &err)
		serializeParamToMap("channel", o.Channel, queryMap, &err)
		serializeParamToMap("before", o.Before, queryMap, &err)
		serializeParamToMap("after", o.After, queryMap, &err)
		serializeParamToMap("with_content", o.WithContent, queryMap, &err)
		serializeParamToMap("tag", o.Tag, queryMap, &err)
		serializeParamToMap("event_types", o.EventTypes, queryMap, &err)
		if err != nil {
			return nil, err
		}
	}

	return executeRequest[any, models.ListResponseMessageOut](
		ctx,
		message.client,
		"GET",
		"/api/v1/app/{app_id}/msg",
		pathMap,
		queryMap,
		nil,
		nil,
	)

}

// Creates a new message and dispatches it to all of the application's endpoints.
//
// The `eventId` is an optional custom unique ID. It's verified to be unique only up to a day, after that no verification will be made.
// If a message with the same `eventId` already exists for the application, a 409 conflict error will be returned.
//
// The `eventType` indicates the type and schema of the event. All messages of a certain `eventType` are expected to have the same schema. Endpoints can choose to only listen to specific event types.
// Messages can also have `channels`, which similar to event types let endpoints filter by them. Unlike event types, messages can have multiple channels, and channels don't imply a specific message content or schema.
//
// The `payload` property is the webhook's body (the actual webhook message). Svix supports payload sizes of up to ~350kb, though it's generally a good idea to keep webhook payloads small, probably no larger than 40kb.
func (message *Message) Create(
	ctx context.Context,
	appId string,
	messageIn models.MessageIn,
	o *MessageCreateOptions,
) (*models.MessageOut, error) {
	pathMap := map[string]string{
		"app_id": appId,
	}
	queryMap := map[string]string{}
	headerMap := map[string]string{}
	var err error
	if o != nil {
		serializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		serializeParamToMap("with_content", o.WithContent, queryMap, &err)
		if err != nil {
			return nil, err
		}
	}

	return executeRequest[models.MessageIn, models.MessageOut](
		ctx,
		message.client,
		"POST",
		"/api/v1/app/{app_id}/msg",
		pathMap,
		queryMap,
		headerMap,
		&messageIn,
	)

}

// Get a message by its ID or eventID.
func (message *Message) Get(
	ctx context.Context,
	appId string,
	msgId string,
	o *MessageGetOptions,
) (*models.MessageOut, error) {
	pathMap := map[string]string{
		"app_id": appId,
		"msg_id": msgId,
	}
	queryMap := map[string]string{}
	var err error
	if o != nil {
		serializeParamToMap("with_content", o.WithContent, queryMap, &err)
		if err != nil {
			return nil, err
		}
	}

	return executeRequest[any, models.MessageOut](
		ctx,
		message.client,
		"GET",
		"/api/v1/app/{app_id}/msg/{msg_id}",
		pathMap,
		queryMap,
		nil,
		nil,
	)

}

// Delete the given message's payload.
//
// Useful in cases when a message was accidentally sent with sensitive content.
// The message can't be replayed or resent once its payload has been deleted or expired.
func (message *Message) ExpungeContent(
	ctx context.Context,
	appId string,
	msgId string,
) error {
	pathMap := map[string]string{
		"app_id": appId,
		"msg_id": msgId,
	}

	_, err := executeRequest[any, any](
		ctx,
		message.client,
		"DELETE",
		"/api/v1/app/{app_id}/msg/{msg_id}/content",
		pathMap,
		nil,
		nil,
		nil,
	)
	return err

}
