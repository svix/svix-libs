package svix

import (
	"fmt"
	"net/http"
	"net/url"
	"strings"
	"time"
)

type (
	SvixOptions struct {
		ServerUrl     *url.URL
		HTTPClient    *http.Client
		RetrySchedule *[]time.Duration
		Debug         bool
	}
	Svix struct {
		Authentication             *Authentication
		Application                *Application
		Endpoint                   *Endpoint
		EventType                  *EventType
		Integration                *Integration
		Message                    *Message
		MessageAttempt             *MessageAttempt
		Statistics                 *Statistics
		OperationalWebhookEndpoint *OperationalWebhookEndpoint
	}
)

func New(token string, options *SvixOptions) (*Svix, error) {
	svixHttpClient := defaultSvixHttpClient(getDefaultBaseUrl(token))

	if options != nil {
		if options.ServerUrl != nil {
			svixHttpClient.BaseURL = options.ServerUrl.String()
		}
		if options.RetrySchedule != nil {
			if len(*options.RetrySchedule) > 5 {
				return nil, fmt.Errorf("number of retries must not exceed 5")
			}
			svixHttpClient.RetrySchedule = *options.RetrySchedule

		}
		if options.HTTPClient != nil {
			svixHttpClient.HTTPClient = options.HTTPClient
		}
		svixHttpClient.Debug = options.Debug

	}

	svixHttpClient.DefaultHeaders["Authorization"] = fmt.Sprintf("Bearer %s", token)
	svixHttpClient.DefaultHeaders["User-Agent"] = fmt.Sprintf("svix-libs/%s/go", Version)

	svx := Svix{
		Authentication: &Authentication{
			client: &svixHttpClient,
		},
		Application: &Application{
			client: &svixHttpClient,
		},
		Endpoint: &Endpoint{
			client: &svixHttpClient,
		},
		EventType: &EventType{
			client: &svixHttpClient,
		},
		Message: &Message{
			client: &svixHttpClient,
		},
		Integration: &Integration{
			client: &svixHttpClient,
		},
		MessageAttempt: &MessageAttempt{
			client: &svixHttpClient,
		},
		Statistics: &Statistics{
			client: &svixHttpClient,
		},
		OperationalWebhookEndpoint: &OperationalWebhookEndpoint{
			client: &svixHttpClient,
		},
	}
	return &svx, nil
}

func getDefaultBaseUrl(token string) string {
	var tokenParts = strings.Split(token, ".")
	var region = tokenParts[len(tokenParts)-1]
	if region == "us" {
		return "https://api.us.svix.com"
	} else if region == "eu" {
		return "https://api.eu.svix.com"
	} else if region == "in" {
		return "https://api.in.svix.com"
	} else {
		return "https://api.svix.com"
	}

}
