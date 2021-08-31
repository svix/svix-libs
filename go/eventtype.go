package svix

import (
	"context"

	"github.com/svix/svix-libs/go/internal/openapi"
)

type EventType struct {
	api *openapi.APIClient
}

type (
	ListResponseEventTypeOut openapi.ListResponseEventTypeOut
	EventTypeIn              openapi.EventTypeIn
	EventTypeOut             openapi.EventTypeOut
	EventTypeUpdate          openapi.EventTypeUpdate
)

type EventTypeListOptions struct {
	Iterator    *string
	Limit       *int32
	WithContent *bool
}

func (e *EventType) List(options *EventTypeListOptions) (*ListResponseEventTypeOut, error) {
	req := e.api.EventTypeApi.ListEventTypesApiV1EventTypeGet(context.Background())
	if options != nil {
		if options.Iterator != nil {
			req = req.Iterator(*options.Iterator)
		}
		if options.Limit != nil {
			req = req.Limit(*options.Limit)
		}
		if options.WithContent != nil {
			req = req.WithContent(*options.WithContent)
		}
	}
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := ListResponseEventTypeOut(out)
	return &ret, nil
}

func (e *EventType) Create(eventTypeIn *EventTypeIn) (*EventTypeOut, error) {
	req := e.api.EventTypeApi.CreateEventTypeApiV1EventTypePost(context.Background())
	req = req.EventTypeIn(openapi.EventTypeIn(*eventTypeIn))
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := EventTypeOut(out)
	return &ret, nil
}

func (e *EventType) Update(eventTypeName string, eventTypeUpdate *EventTypeUpdate) (*EventTypeOut, error) {
	req := e.api.EventTypeApi.UpdateEventTypeApiV1EventTypeEventTypeNamePut(context.Background(), eventTypeName)
	req = req.EventTypeUpdate(openapi.EventTypeUpdate(*eventTypeUpdate))
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := EventTypeOut(out)
	return &ret, nil
}

func (e *EventType) Delete(eventTypeName string) error {
	req := e.api.EventTypeApi.DeleteEventTypeApiV1EventTypeEventTypeNameDelete(context.Background(), eventTypeName)
	res, err := req.Execute()
	return wrapError(err, res)
}
