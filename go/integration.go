package svix

import (
	"context"

	"github.com/svix/svix-libs/go/internal/openapi"
)

type (
	ListResponseIntegrationOut openapi.ListResponseIntegrationOut
	IntegrationIn              openapi.IntegrationIn
	IntegrationUpdate          openapi.IntegrationUpdate
	IntegrationOut             openapi.IntegrationOut
	IntegrationKeyOut          openapi.IntegrationKeyOut
)

type Integration struct {
	api *openapi.APIClient
}

type IntegrationListOptions struct {
	Iterator *string
	Limit    *int32
}

func (e *Integration) List(appId string, options *IntegrationListOptions) (*ListResponseIntegrationOut, error) {
	req := e.api.IntegrationApi.ListIntegrationsApiV1AppAppIdIntegrationGet(context.Background(), appId)
	if options != nil {
		if options.Iterator != nil {
			req = req.Iterator(*options.Iterator)
		}
		if options.Limit != nil {
			req = req.Limit(*options.Limit)
		}
	}
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := ListResponseIntegrationOut(out)
	return &ret, nil
}

func (e *Integration) Create(appId string, endpointIn *IntegrationIn) (*IntegrationOut, error) {
	req := e.api.IntegrationApi.CreateIntegrationApiV1AppAppIdIntegrationPost(context.Background(), appId)
	req = req.IntegrationIn(openapi.IntegrationIn(*endpointIn))
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := IntegrationOut(out)
	return &ret, nil
}

func (e *Integration) Get(appId string, integId string) (*IntegrationOut, error) {
	req := e.api.IntegrationApi.GetIntegrationApiV1AppAppIdIntegrationIntegIdGet(context.Background(), integId, appId)
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := IntegrationOut(out)
	return &ret, nil
}

func (e *Integration) Update(appId string, integId string, endpointUpdate *IntegrationUpdate) (*IntegrationOut, error) {
	req := e.api.IntegrationApi.UpdateIntegrationApiV1AppAppIdIntegrationIntegIdPut(context.Background(), integId, appId)
	req = req.IntegrationUpdate(openapi.IntegrationUpdate(*endpointUpdate))
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := IntegrationOut(out)
	return &ret, nil
}

func (e *Integration) Delete(appId string, integId string) error {
	req := e.api.IntegrationApi.DeleteIntegrationApiV1AppAppIdIntegrationIntegIdDelete(context.Background(), integId, appId)
	res, err := req.Execute()
	return wrapError(err, res)
}

func (e *Integration) GetKey(appId string, integId string) (*IntegrationKeyOut, error) {
	req := e.api.IntegrationApi.GetIntegrationKeyApiV1AppAppIdIntegrationIntegIdKeyGet(context.Background(), integId, appId)
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := IntegrationKeyOut(out)
	return &ret, nil
}

func (e *Integration) RotateKey(appId string, integId string) (*IntegrationKeyOut, error) {
	req := e.api.IntegrationApi.RotateIntegrationKeyApiV1AppAppIdIntegrationIntegIdKeyRotatePost(context.Background(), integId, appId)
	out, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	ret := IntegrationKeyOut(out)
	return &ret, nil
}
